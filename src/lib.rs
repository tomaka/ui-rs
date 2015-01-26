#![feature(unsafe_destructor)]
#![warn(missing_docs)]
#![unstable]

extern crate nalgebra;

use std::default::Default;
use nalgebra::Vec2;

pub use components::{ButtonComponent, TextComponent};
pub use shape::Shape;

mod components;
mod shape;

/// Indicates how a component should be displayed.
pub enum RenderOutput<'a> {
    HorizontalBox {
        children: Vec<RenderOutput<'a>>,
    },

    Component(&'a Component),

    /// The component should draw a shape. Note that the position of the shape is relative to
    /// the position of the component on the screen.
    ///
    /// **Important**: if you manually return shapes, your component should also implement
    /// `get_dimensions` and `get_bounding_box`.
    Shape(Shape),
}

pub trait Component: Send + Sync + 'static {
    /// Returns the list of things that must be drawn.
    fn render(&self) -> RenderOutput;

    /// Returns the dimensions of the component. If returns `None`, the dimensions are
    /// automatically calculated using what `render` returns.
    ///
    /// If `render` only returns shapes, then your component will have a dimension of `(0.0, 0.0)`.
    ///
    /// The dimensions are used when calculating layouts.
    fn get_dimensions(&self) -> Option<Vec2<f32>> {
        None
    }

    fn get_bounding_box(&self) -> Option<((u32, u32), (u32, u32))> {
        None
    }
}

/// The main struct of this library. Manages the whole user interface.
pub struct Ui<T> {
    main_component: T,
    shapes: Vec<Shape>,
}

/// Allows mutable access to the main component of the `Ui`.
pub struct UiMainComponentMutRef<'a, T: 'a> {
    ui: &'a mut Ui<T>,
}

impl<T> Ui<T> where T: Component {
    pub fn new(component: T) -> Ui<T> {
        let mut ui = Ui {
            main_component: component,
            shapes: Vec::new(),
        };

        ui.update();
        ui
    }

    /// Sets the viewport of the user interface.
    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.update();
    }

    /// Changes the position of the mouse over the UI.
    pub fn set_mouse_position(&mut self, x: u32, y: u32) {
        self.update();
    }

    /// Gives a mutable access to the main component in order for you to modify it.
    pub fn get_mut_main_component(&mut self) -> UiMainComponentMutRef<T> {
        UiMainComponentMutRef { ui: self }
    }

    /// Returns the list of all the shapes that must be drawn as part of this UI.
    ///
    /// The list is sorted from bottom-z-to-top.
    pub fn draw(&self) -> &[Shape] {
        &self.shapes[]
    }

    fn update(&mut self) {
        let (shapes, _) = process(self.main_component.render(), Vec2::new(0.0, 0.0));
        self.shapes = shapes;
    }
}

fn process(output: RenderOutput, mut current_position: Vec2<f32>) -> (Vec<Shape>, Vec2<f32>) {
    match output {
        RenderOutput::HorizontalBox { children } => {
            let mut shapes = Vec::new();
            let mut max_height = 0.0;
            for child in children.into_iter() {
                let (child_shapes, child_dims) = process(child, current_position);
                max_height = std::cmp::partial_max(max_height, child_dims.y).unwrap_or(max_height);
                shapes.extend(child_shapes.into_iter());
                current_position.x += child_dims.x;
            }
            (shapes, current_position)
        },
        RenderOutput::Component(child) => {
            let dimensions = child.get_dimensions();
            let (shapes, alt_dims) = process(child.render(), current_position);
            let dimensions = dimensions.unwrap_or(alt_dims);
            (shapes, dimensions)
        },
        RenderOutput::Shape(shape) => {
            let shape = shape.translate(current_position);
            (vec![shape], Vec2::new(0.0, 0.0))
        },
    }
}

impl<'a, T> std::ops::Deref for UiMainComponentMutRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ui.main_component
    }
}

impl<'a, T> std::ops::DerefMut for UiMainComponentMutRef<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ui.main_component
    }
}

#[unsafe_destructor]
impl<'a, T: 'a> Drop for UiMainComponentMutRef<'a, T> where T: Component {
    fn drop(&mut self) {
        self.ui.update();
    }
}
