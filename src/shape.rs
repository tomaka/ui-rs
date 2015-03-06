use std::default::Default;
use nalgebra::{Vec2};

/// A shape that can be drawn on the screen.
///
/// The positions and sizes are in logical coords. `(-1.0, -1.0)` corresponds to the bottom-left
/// hand corner of the screen, and `(1.0, 1.0)` corresponds to the top-right hand corner.
#[derive(Debug)]
pub enum Shape {
    /// A single point.
    Point {
        /// Position of the center of the point.
        location: Vec2<f32>,
        /// Color of the point.
        color: [f32; 3],
    },

    /// A single line.
    Line {
        /// Position of one extremity of the line.
        from: Vec2<f32>,
        /// Position of the other extremity of the line.
        to: Vec2<f32>,
        /// Color of the line.
        color: [f32; 3],
    },

    /// A rectangle. Can only be perpendicular to the screen.
    Rectangle {
        /// Position of one edge of the rectangle.
        from: Vec2<f32>,
        /// Position of the other edge of the rectangle.
        to: Vec2<f32>,
        /// Color of the rectangle.
        color: [f32; 3],
    },

    /// An image. Can only be perpendicular to the screen.
    Image {
        /// Position of one edge of the rectangle.
        from: Vec2<f32>,
        /// Position of the other edge of the rectangle.
        to: Vec2<f32>,
        /// Image to draw.
        image: Image,
    },

    /// A text. Can only be perpendicular to the screen.
    Text {
        /// The text to write.
        text: String,
        /// Font to use.
        font: Font,
        /// Position of the bottom-left hand corner of the first letter of the text.
        bottom_left: Vec2<f32>,
        /// Size of one EM of text.
        em: f32,
    },
}

/// All the possible images.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Image {
    /// The image corresponding to a regular button.
    UnhoveredButton,

    /// The image corresponding to a button behing hovered.
    HoveredButton,

    /// A custom image. Not used by this library's predefined components.
    Custom(String),
}

/// All the possible fonts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Font {
    /// The default font for regular text.
    Default,

    /// The font to use for the label over a button.
    Button,

    /// A custom font. Not used by this library's predefined components.
    Custom(String),
}

impl Default for Font {
    fn default() -> Font {
        Font::Default
    }
}

impl Shape {
    /// Moves a shape by the given coordinates.
    pub fn translate(mut self, vec: Vec2<f32>) -> Shape {
        match &mut self {
            &mut Shape::Point { ref mut location, .. } => {
                *location = location.clone() + vec;
            },

            &mut Shape::Line { ref mut from, ref mut to, .. } => {
                *from = from.clone() + vec;
                *to = to.clone() + vec;
            },

            &mut Shape::Rectangle { ref mut from, ref mut to, .. } => {
                *from = from.clone() + vec;
                *to = to.clone() + vec;
            },

            &mut Shape::Image { ref mut from, ref mut to, .. } => {
                *from = from.clone() + vec;
                *to = to.clone() + vec;
            },

            &mut Shape::Text { ref mut bottom_left, .. } => {
                *bottom_left = bottom_left.clone() + vec;
            },
        }

        self
    }
}
