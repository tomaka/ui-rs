use std::any::Any;
use std::default::Default;
use nalgebra::Vec2;

use predefined::TextComponent;
use shape::{Shape, Font};
use component::RawComponent;

pub struct ButtonComponent {
    color: [f32; 3],
    label: TextComponent,
    hovered: bool,
    can_be_pressed: bool,
    previous_pressed_status: bool,
}

#[derive(Debug, Clone)]
pub struct PressedEvent;

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
            can_be_pressed: false,
            previous_pressed_status: false,
        }
    }
}

impl RawComponent for ButtonComponent {
    fn render(&mut self) -> Vec<Shape> {
        vec![Shape::Rectangle {
            from: Vec2::new(0.0, 0.0),
            to: Vec2::new(0.1, 0.1),
            color: if self.hovered {
                [self.color[0] * 0.8, self.color[1] * 0.8, self.color[2] * 0.8]
            } else {
                self.color
            },
        }]
    }

    fn set_mouse_status(&mut self, position: Option<Vec2<f32>>, pressed: bool) -> Vec<Box<Any>> {
        let mut ret = if position.is_some() {
            self.hovered = true;
            Vec::with_capacity(0)

        } else {
            self.hovered = false;
            Vec::with_capacity(0)
        };

        if position.is_some() && self.can_be_pressed &&
            self.previous_pressed_status == true && pressed == false
        {
            ret.push(Box::new(PressedEvent) as Box<Any>);
        }

        if position.is_none() {
            self.can_be_pressed = false;
        }

        if position.is_some() && pressed == false {
            self.can_be_pressed = true;
        }

        self.previous_pressed_status = pressed;
        ret
    }

    fn hit_test(&mut self, pos: Vec2<f32>) -> bool {
        pos.x >= 0.0 && pos.x < self.get_width() && pos.y >= 0.0 && pos.y < 0.1
    }

    fn get_width(&mut self) -> f32 {
        0.1     // TODO:
    }

    fn get_height(&mut self) -> f32 {
        0.1     // TODO:
    }

    fn handle_raw_child_event(&mut self, _: usize, _: Box<Any>) -> Option<Box<Any>> {
        unreachable!();
    }
}
