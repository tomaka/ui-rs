use nalgebra::{Vec2};
use std::default::Default;
use shape::{Font, Shape};

use component::RawComponent;

#[derive(Default)]
pub struct TextComponent {
    text: String,
    font: Font,
    em: f32,
}

impl TextComponent {
    pub fn new(text: String, font: Font, em: f32) -> TextComponent {
        TextComponent {
            text: text,
            font: font,
            em: em,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn set_em(&mut self, em: f32) {
        self.em = em;
    }

    fn get_dimensions(&self) -> Vec2<f32> {
        // FIXME:
        let width = self.em * self.text.len() as f32;
        Vec2::new(width, self.em)
    }
}

impl RawComponent for TextComponent {
    fn render(&self) -> Vec<Shape> {
        vec![]      // FIXME: 
    }

    fn set_mouse_position(&mut self, _: Option<Vec2<f32>>) {
    }

    fn hit_test(&self, pos: Vec2<f32>) -> bool {
        true
    }
}
