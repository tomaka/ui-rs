extern crate ui;

#[derive(Default)]
pub struct MyWidget {
    left_button: ui::ButtonComponent,
    text: ui::TextComponent,
    right_button: ui::ButtonComponent,
}

impl MyWidget {
    pub fn set_number(&mut self, num: i32) {
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
    let mut ui = ui::Ui::new(<MyWidget as ::std::default::Default>::default());
    ui.get_mut_main_component().set_number(3);

    println!("{:?}", ui.draw());
}
