#![feature(unsafe_destructor)]
#![warn(missing_docs)]
#![unstable]

extern crate nalgebra;

use std::default::Default;

pub use nalgebra::Vec2;
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

    /// Returns the bounding box of the component. If returns `None`, the bounding box corresponds
    /// to the bounding boxes of what `render` returns. The default behavior of this function is
    /// to return `None`, and it is what you should usually do.
    ///
    /// The bounding box is used to determine whether the cursor is hovering the component.
    fn get_bounding_box(&self) -> Option<(Vec2<f32>, Vec2<f32>)> {
        None
    }
}

/// The main struct of this library. Manages the whole user interface.
pub struct Ui<T> {
    main_component: T,
    shapes: Vec<Shape>,
    viewport: Vec2<u32>,
    mouse: Option<Vec2<u32>>,
}

/// Allows mutable access to the main component of the `Ui`.
pub struct UiMainComponentMutRef<'a, T: 'a> {
    ui: &'a mut Ui<T>,
}

impl<T> Ui<T> where T: Component {
    pub fn new(component: T, viewport: Vec2<u32>) -> Ui<T> {
        let mut ui = Ui {
            main_component: component,
            shapes: Vec::new(),
            viewport: viewport,
            mouse: None,
        };

        ui.update();
        ui
    }

    /// Sets the viewport of the user interface.
    pub fn set_viewport(&mut self, dimensions: Vec2<u32>) {
        self.viewport = dimensions;
        self.update();
    }

    /// Changes the position of the mouse over the UI.
    pub fn set_mouse_position(&mut self, position: Option<Vec2<u32>>) {
        self.mouse = position;
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
        let mouse = self.mouse.as_ref().map(|&coords| {
            Vec2::new(
                -1.0 + 2.0 * coords.x as f32 / self.viewport.x as f32,
                1.0 + -2.0 * coords.y as f32 / self.viewport.y as f32,
            )
        }).unwrap_or(Vec2::new(-1.0, -1.0));

        let (shapes, _, hierarchy) = process(self.main_component.render(), Vec2::new(0.0, 0.0),
                                             mouse);
        self.shapes = shapes;

        println!("{}", hierarchy.len());
    }
}

/// Returns a list of shapes to draw, the dimensions of the output, and the hierarchy of components
/// that are under the mouse cursor from inner to outter.
fn process<'a>(output: RenderOutput<'a>, mut current_position: Vec2<f32>, mouse: Vec2<f32>)
               -> (Vec<Shape>, Vec2<f32>, Vec<&'a Component>)
{
    match output {
        RenderOutput::HorizontalBox { children } => {
            let mut shapes = Vec::new();
            let mut max_height = 0.0;
            let mut main_hierarchy = None;

            for child in children.into_iter() {
                let (child_shapes, child_dims, hierarchy) = process(child, current_position, mouse);
                if hierarchy.len() != 0 {
                    main_hierarchy = Some(hierarchy);
                }

                max_height = std::cmp::partial_max(max_height, child_dims.y).unwrap_or(max_height);
                shapes.extend(child_shapes.into_iter());
                current_position.x += child_dims.x;
            }

            (shapes, current_position, main_hierarchy.unwrap_or(vec![]))
        },

        RenderOutput::Component(child) => {
            let dimensions = child.get_dimensions();
            let bounding_box = child.get_bounding_box();
            let (shapes, alt_dims, mut hierarchy) = process(child.render(), current_position, mouse);
            let dimensions = dimensions.unwrap_or(alt_dims);

            let hierarchy = if let Some(bounding_box) = bounding_box {
                if mouse.x >= bounding_box.0.x + current_position.x &&
                    mouse.x <= bounding_box.1.x + current_position.x &&
                    mouse.y >= bounding_box.0.y + current_position.y &&
                    mouse.y <= bounding_box.1.y + current_position.y
                {
                    vec![child]
                } else {
                    vec![]
                }

            } else {
                if hierarchy.len() == 0 {
                    vec![]
                } else {
                    hierarchy.push(child);
                    hierarchy
                }
            };

            (shapes, dimensions, hierarchy)
        },

        RenderOutput::Shape(shape) => {
            let shape = shape.translate(current_position);
            (vec![shape], Vec2::new(0.0, 0.0), vec![])
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
