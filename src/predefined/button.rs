use std::default::Default;
use std::sync::atomic::{AtomicBool, Ordering};
use nalgebra::Vec2;

use predefined::TextComponent;
use shape::{Shape, Font};
use component::RawComponent;

pub struct ButtonComponent {
    color: [f32; 3],
    label: TextComponent,
    hovered: AtomicBool,
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
            hovered: AtomicBool::new(false),
        }
    }
}

impl RawComponent for ButtonComponent {
    fn render(&self) -> Vec<Shape> {
        vec![Shape::Rectangle {
            from: Vec2::new(0.0, 0.0),
            to: Vec2::new(0.1, 0.1),
            color: if self.hovered.load(Ordering::Relaxed) {
                [self.color[0] * 0.8, self.color[1] * 0.8, self.color[2] * 0.8]
            } else {
                self.color
            },
        }]
    }

    fn set_mouse_position(&mut self, position: Option<Vec2<f32>>) {
        if position.is_some() {
            self.hovered.store(true, Ordering::Relaxed);
        } else {
            self.hovered.store(false, Ordering::Relaxed);
        }
    }

    fn hit_test(&self, pos: Vec2<f32>) -> bool {
        true
    }
}
