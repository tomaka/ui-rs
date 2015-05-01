use std::any::Any;
use nalgebra::Vec2;

use shape::Shape;

/// Represents a raw component. Don't implement this directly.
pub trait RawComponent<E> {
    /// Obtain the look of this component.
    fn render(&mut self) -> Vec<Shape>;

    /// Tells the component where the mouse is. `None` if the mouse is not over the element.
    ///
    /// Returns a list of events to pass to the parent.
    fn set_mouse_status(&mut self, Option<Vec2<f32>>, pressed: bool) -> Vec<E>;

    /// Asks the component whether the given position touches it.
    fn hit_test(&mut self, Vec2<f32>) -> bool;

    /// Returns the width of the component.
    fn get_width(&mut self) -> f32;

    /// Returns the height of the component.
    fn get_height(&mut self) -> f32;
}
