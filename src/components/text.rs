use nalgebra::{Vec2};
use std::default::Default;
use Component;
use RenderOutput;
use shape::{Font, Shape};

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
}

impl Component for TextComponent {
    fn render(&self) -> RenderOutput {
        RenderOutput::Shape(
            Shape::Text {
                text: self.text.clone(),
                font: self.font.clone(),
                bottom_left: Vec2::new(0.0, 0.0),
                em: self.em,
            }
        )
    }

    fn get_dimensions(&self) -> Option<Vec2<f32>> {
        // FIXME:
        let width = self.em * self.text.len() as f32;
        Some(Vec2::new(width, self.em))
    }
}
