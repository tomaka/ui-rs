#![feature(unsafe_destructor)]
#![warn(missing_docs)]
#![unstable]

extern crate nalgebra;

use std::default::Default;

pub use nalgebra::Vec2;
pub use components::{ButtonComponent, TextComponent};
pub use shape::Shape;
pub use ui::{Ui, UiMainComponentMutRef};

mod components;
mod shape;
mod ui;

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

    /// Sets whether this component is hovered by the mouse or not.
    ///
    /// The default action is not to do anything.
    fn set_hovered_status(&self, HoveredStatus) { }

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

/// State of a component in regards to the mouse position.
#[derive(Debug, Clone, Copy)]
pub enum HoveredStatus {
    Hovered,
    ChildHovered,
    NotHovered,
}

