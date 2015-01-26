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

    Shape(Shape),
}

pub trait Component: Send + Sync + 'static {
    /// Returns the list of things that must be drawn.
    fn render(&self) -> RenderOutput;

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
    pub fn draw(&self) -> &[Shape] {
        &self.shapes[]
    }

    fn update(&mut self) {
        self.shapes = process(self.main_component.render(), Vec2::new(0.0, 0.0));
    }
}

fn process(output: RenderOutput, mut current_position: Vec2<f32>) -> Vec<Shape> {
    match output {
        RenderOutput::HorizontalBox { children } => {
            let mut shapes = Vec::new();
            for child in children.into_iter() {
                let mut max_width = 0.0;
                for shape in process(child, current_position).into_iter() {
                    max_width = std::cmp::partial_max(max_width, shape.get_width()).unwrap_or(0.0);
                    shapes.push(shape);
                }
                current_position.x += max_width;
            }
            shapes
        },
        RenderOutput::Component(child) => process(child.render(), current_position),
        RenderOutput::Shape(shape) => {
            let shape = shape.translate(current_position);
            vec![shape]
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
