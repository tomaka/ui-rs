use nalgebra::{Vec2};
use std::any::Any;
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

impl<E> RawComponent<E> for TextComponent {
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

    fn set_mouse_status(&mut self, _: Option<Vec2<f32>>, _: bool) -> Vec<E> {
        Vec::with_capacity(0)
    }

    fn hit_test(&mut self, pos: Vec2<f32>) -> bool {
        pos.x >= 0.0 && pos.x < RawComponent::<E>::get_width(self) && pos.y >= 0.0 && pos.y < self.em
    }

    fn get_width(&mut self) -> f32 {
        self.em * self.text.len() as f32
    }

    fn get_height(&mut self) -> f32 {
        self.em
    }
}
