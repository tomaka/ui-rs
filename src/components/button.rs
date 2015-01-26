use std::default::Default;
use nalgebra::Vec2;

use components::TextComponent;
use shape::{Shape, Font};

use Component;
use RenderOutput;

pub struct ButtonComponent {
    color: [f32; 3],
    label: TextComponent,
    hovered: bool,
}

impl ButtonComponent {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
}

impl Default for ButtonComponent {
    fn default() -> ButtonComponent {
        ButtonComponent {
            color: [1.0, 1.0, 0.0],
            label: TextComponent::new("Button".to_string(), Font::Button, 0.1),
            hovered: false,
        }
    }
}

impl Component for ButtonComponent {
    fn render(&self) -> RenderOutput {
        RenderOutput::Shape(Shape::Rectangle {
            from: Vec2::new(0.0, 0.0),
            to: Vec2::new(0.1, 0.1),
            color: if self.hovered {
                [self.color[0] * 0.8, self.color[1] * 0.8, self.color[2] * 0.8]
            } else {
                self.color
            },
        })
    }

    fn get_dimensions(&self) -> Option<Vec2<f32>> {
        // FIXME:
        Some(Vec2::new(0.1, 0.1))
    }

    fn get_bounding_box(&self) -> Option<(Vec2<f32>, Vec2<f32>)> {
        Some((Vec2::new(0.0, 0.0), Vec2::new(0.1, 0.1)))
    }
}
