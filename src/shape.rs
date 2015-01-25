use std::default::Default;
use nalgebra::{Vec2};

#[derive(Debug)]
pub enum Shape {
    Point {
        location: Vec2<f32>,
        color: [f32; 3],
    },

    Line {
        from: Vec2<f32>,
        to: Vec2<f32>,
        color: [f32; 3],
    },

    Rectangle {
        from: Vec2<f32>,
        to: Vec2<f32>,
        color: [f32; 3],
    },

    Image {
        from: Vec2<f32>,
        to: Vec2<f32>,
        image: Image,
    },

    Text {
        text: String,
        font: Font,
        bottom_left: Vec2<f32>,
        em: f32,
    },
}

/// All the possible images.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Image {
    UnhoveredButton,
    HoveredButton,
    Custom(String),
}

/// All the possible fonts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Font {
    Default,
    Button,
    Custom(String),
}

impl Default for Font {
    fn default() -> Font {
        Font::Default
    }
}

impl Shape {
    pub fn get_width(&self) -> f32 {
        match self {
            &Shape::Point { .. } => unimplemented!(),
            &Shape::Line { ref from, ref to, .. } => {
                to.x - from.x
            },
            &Shape::Rectangle { ref from, ref to, .. } => {
                to.x - from.x
            },
            &Shape::Image { ref from, ref to, .. } => {
                to.x - from.x
            },
            &Shape::Text { ref text, em, .. } => {
                // FIXME: 
                (text.len() as f32) * em
            },
        }
    }

    pub fn get_height(&self) -> f32 {
        match self {
            &Shape::Point { .. } => unimplemented!(),
            &Shape::Line { ref from, ref to, .. } => {
                to.y - from.y
            },
            &Shape::Rectangle { ref from, ref to, .. } => {
                to.y - from.y
            },
            &Shape::Image { ref from, ref to, .. } => {
                to.y - from.y
            },
            &Shape::Text { em, .. } => {
                em
            },
        }
    }

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
