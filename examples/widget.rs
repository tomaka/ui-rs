extern crate glium;
extern crate glium_renderer;
extern crate glutin;
extern crate ui;

use std::default::Default;
use glium::Surface;

pub struct MyWidget {
    left_button: ui::predefined::ButtonComponent,
    text: ui::predefined::TextComponent,
    right_button: ui::predefined::ButtonComponent,
}

impl Default for MyWidget {
    fn default() -> MyWidget {
        MyWidget {
            left_button: Default::default(),
            right_button: Default::default(),
            text: ui::predefined::TextComponent::new("0".to_string(), Default::default(), 0.1),
        }
    }
}

impl MyWidget {
    pub fn set_number(&mut self, num: i32) {
        self.text.set_text(format!("{}", num));
    }
}

impl ui::Component for MyWidget {
    type EmittedEvent = ();
    type ReceivedEvent = ();
}

fn main() {
    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let dimensions = display.get_framebuffer_dimensions();

    let system = glium_renderer::UiSystem::new(&display);

    let mut ui = ui::Ui::new(<MyWidget as ::std::default::Default>::default(), ui::Vec2::new(dimensions.0, dimensions.1));
    ui.get_mut_main_component().set_number(3);

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        system.draw(&mut target, &ui);
        target.finish();

        match display.wait_events().next().unwrap() {
            glutin::Event::Closed => break 'main,
            glutin::Event::Resized(w, h) => {
                ui.set_viewport(ui::Vec2::new(w, h));
            },
            glutin::Event::MouseMoved((x, y)) => {
                ui.set_mouse_position(Some(ui::Vec2::new(x as u32, y as u32)));
            },
            _ => ()
        }
    }
}
