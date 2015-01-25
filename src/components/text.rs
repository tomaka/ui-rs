use nalgebra::{Vec2};
use std::default::Default;
use Component;
use RenderOutput;
use shape::{Font, Shape};

#[derive(Default)]
pub struct TextComponent {
    text: String,
    font: Font,
}

impl TextComponent {
    pub fn new(text: String, font: Font) -> TextComponent {
        TextComponent {
            text: text,
            font: font,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }
}

impl Component for TextComponent {
    fn render(&self) -> RenderOutput {
        RenderOutput::Shape(
            Shape::Text {
                text: self.text.clone(),
                font: self.font.clone(),
                bottom_left: Vec2::new(0.0, 0.0),
                em: 1.0,
            }
        )
    }
}
