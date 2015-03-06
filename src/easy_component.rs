use nalgebra::Vec2;

use component::RawComponent;
use Shape;

pub trait Component: Send + Sync + 'static {
    /// What events type this component produces.
    type EmittedEvent: 'static = ();

    /// What events type this component expects to receive.
    type ReceivedEvent: 'static = ();

    /// A child has produced an event.
    fn handle_child_event(&mut self, Self::ReceivedEvent) {
    }

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

impl<T> RawComponent for T where T: Component {
    fn render(&self) -> Vec<Shape> {
        vec![]
    }

    fn set_mouse_position(&mut self, _: Option<Vec2<f32>>) {
    }

    fn hit_test(&self, _: Vec2<f32>) -> bool {
        false
    }
}

/// State of a component in regards to the mouse position.
#[derive(Debug, Clone, Copy)]
pub enum HoveredStatus {
    Hovered,
    ChildHovered,
    NotHovered,
}
