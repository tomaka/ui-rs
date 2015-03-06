use nalgebra::Vec2;

use shape::Shape;

/// Represents a raw component. Don't implement this directly.
pub trait RawComponent {
    /// Obtain the look of this component.
    fn render(&self) -> Vec<Shape>;

    /// Tells the component where the mouse is.
    ///
    /// `None` if the mouse is not over the element.
    fn set_mouse_position(&mut self, Option<Vec2<f32>>);

    /// Asks the component whether the given position touches it.
    fn hit_test(&self, Vec2<f32>) -> bool;

    /// Returns the width of the component.
    fn get_width(&self) -> f32;
}
