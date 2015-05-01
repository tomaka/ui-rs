extern crate glium;
extern crate glium_renderer;
extern crate glutin;
extern crate ui;

use std::default::Default;
use glium::Surface;

mod left_bar;
mod main_ui;

fn main() {
    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let dimensions = display.get_framebuffer_dimensions();

    let system = glium_renderer::UiSystem::new(&display);

    let mut ui: ui::Ui<_, main_ui::MainUiEvent> = ui::Ui::new(<main_ui::MainUi as Default>::default(), ui::Vec2::new(dimensions.0, dimensions.1));

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
