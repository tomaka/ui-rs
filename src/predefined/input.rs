use nalgebra::{Vec2};
use std::any::Any;
use std::default::Default;
use shape::{Font, Shape};

use component::RawComponent;

#[derive(Default)]
pub struct InputComponent {
    text: String,
    font: Font,
    em: f32,
}

impl InputComponent {
    pub fn new(text: String, font: Font, em: f32) -> InputComponent {
        InputComponent {
            text: text,
            font: font,
            em: em,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn set_em(&mut self, em: f32) {
        self.em = em;
    }
}

impl RawComponent for InputComponent {
    fn render(&mut self) -> Vec<Shape> {
        vec![
            Shape::Text {
                text: self.text.clone(),
                font: self.font.clone(),
                bottom_left: Vec2::new(0.0, 0.0),
                em: self.em,
            }
        ]
    }

    fn set_mouse_status(&mut self, position: Option<Vec2<f32>>, pressed: bool) -> Vec<Box<Any>> {
        if position.is_some() && pressed {
            
        }

        Vec::with_capacity(0)
    }

    fn hit_test(&mut self, pos: Vec2<f32>) -> bool {
        pos.x >= 0.0 && pos.x < self.get_width() && pos.y >= 0.0 && pos.y < self.em
    }

    fn get_width(&mut self) -> f32 {
        self.em * self.text.len() as f32
    }

    fn get_height(&mut self) -> f32 {
        self.em
    }

    fn handle_raw_child_event(&mut self, _: usize, _: Box<Any>) -> Option<Box<Any>> {
        unreachable!();
    }
}
