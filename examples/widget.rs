extern crate glium;
extern crate glium_renderer;
extern crate glutin;
extern crate ui;

use std::default::Default;
use glium::Surface;

struct MyWidget {
    value: i32,
    left_button: ui::predefined::ButtonComponent,
    text: ui::predefined::TextComponent,
    right_button: ui::predefined::ButtonComponent,
}

impl Default for MyWidget {
    fn default() -> MyWidget {
        MyWidget {
            value: 0,
            left_button: Default::default(),
            right_button: Default::default(),
            text: ui::predefined::TextComponent::new("0".to_string(), Default::default(), 0.1),
        }
    }
}

impl MyWidget {
    fn get_number(&self) -> i32 {
        self.value
    }

    fn set_number(&mut self, num: i32) {
        self.value = num;
        self.text.set_text(format!("{}", num));
    }
}

struct MyWidgetEvent;

impl ui::Component for MyWidget {
    type EmittedEvent = MyWidgetEvent;
    type ReceivedEvent = ui::predefined::button::ButtonEvent;

    fn get_layout(&mut self) -> ui::Layout<ui::predefined::button::ButtonEvent> {
        ui::Layout::HorizontalBox(vec![&mut self.left_button, &mut self.text, &mut self.right_button])
    }

    fn handle_child_event(&mut self, child_id: usize, _: ui::predefined::button::ButtonEvent)
                          -> Option<MyWidgetEvent>
    {
        if child_id == 0 {
            let value = self.value;
            self.set_number(value - 1);

        } else if child_id == 2 {
            let value = self.value;
            self.set_number(value + 1);
        }

        Some(MyWidgetEvent)
    }
}

struct MyWidgetWithWidgets {
    widgets: Vec<MyWidget>,
    text: ui::predefined::TextComponent,
}

impl Default for MyWidgetWithWidgets {
    fn default() -> MyWidgetWithWidgets {
        MyWidgetWithWidgets {
            widgets: vec![Default::default(), Default::default(), Default::default()],
            text: ui::predefined::TextComponent::new("0".to_string(), Default::default(), 0.1),
        }
    }
}

impl ui::Component for MyWidgetWithWidgets {
    type EmittedEvent = ();
    type ReceivedEvent = MyWidgetEvent;

    fn get_layout(&mut self) -> ui::Layout<MyWidgetEvent> {
        let mut b: Vec<_> = self.widgets.iter_mut().map(|w| w as &mut ui::component::RawComponent<_>).collect();
        b.push(&mut self.text);
        ui::Layout::VerticalBox(b)
    }

    fn handle_child_event(&mut self, _: usize, _: MyWidgetEvent) -> Option<()> {
        let val = self.widgets.iter().map(|w| w.get_number()).fold(0, |a, e| a + e);
        self.text.set_text(format!("{}", val));
        None
    }
}

fn main() {
    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let dimensions = display.get_framebuffer_dimensions();

    let system = glium_renderer::UiSystem::new(&display);

    let mut ui: ui::Ui<_, ()> = ui::Ui::new(<MyWidgetWithWidgets as Default>::default(), ui::Vec2::new(dimensions.0, dimensions.1));

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
            glutin::Event::MouseInput(glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
                ui.set_mouse_pressed(true);
            },
            glutin::Event::MouseInput(glutin::ElementState::Released, glutin::MouseButton::Left) => {
                ui.set_mouse_pressed(false);
            },
            _ => ()
        }
    }
}
