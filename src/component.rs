use std::any::Any;
use nalgebra::Vec2;

use shape::Shape;

/// Represents a raw component. Don't implement this directly.
pub trait RawComponent {
    /// Obtain the look of this component.
    fn render(&mut self) -> Vec<Shape>;

    /// Handles an event received by a child and returns an event to propagate to its parent.
    fn handle_raw_child_event(&mut self, child_id: usize, event: Box<Any>) -> Option<Box<Any>>;

    /// Tells the component where the mouse is. `None` if the mouse is not over the element.
    ///
    /// Returns a list of events to pass to the parent.
    fn set_mouse_position(&mut self, Option<Vec2<f32>>) -> Vec<Box<Any>>;

    /// Asks the component whether the given position touches it.
    fn hit_test(&mut self, Vec2<f32>) -> bool;

    /// Returns the width of the component.
    fn get_width(&mut self) -> f32;

    /// Returns the height of the component.
    fn get_height(&mut self) -> f32;
}
