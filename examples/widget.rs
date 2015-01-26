extern crate glium;
extern crate glium_renderer;
extern crate glutin;
extern crate ui;

use glium::Surface;

#[derive(Default)]
pub struct MyWidget {
    left_button: ui::ButtonComponent,
    text: ui::TextComponent,
    right_button: ui::ButtonComponent,
}

impl MyWidget {
    pub fn set_number(&mut self, num: i32) {
        self.text.set_em(0.1);
        self.text.set_text(format!("{}", num));
    }
}

impl ui::Component for MyWidget {
    fn render(&self) -> ui::RenderOutput {
        ui::RenderOutput::HorizontalBox {
            children: vec![
                ui::RenderOutput::Component(&self.left_button),
                ui::RenderOutput::Component(&self.text),
                ui::RenderOutput::Component(&self.right_button),
            ]
        }
    }
}

fn main() {
    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let system = glium_renderer::UiSystem::new(&display);

    let mut ui = ui::Ui::new(<MyWidget as ::std::default::Default>::default());
    ui.get_mut_main_component().set_number(3);

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        system.draw(&mut target, &ui);
        target.finish();

        for event in display.wait_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
