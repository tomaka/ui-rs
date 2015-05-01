use std::default::Default;

use ui;
use ui::predefined;
use ui::component::RawComponent;

pub struct LeftBarWidget {
    buttons: Vec<predefined::ButtonComponent>,
}

impl Default for LeftBarWidget {
    fn default() -> LeftBarWidget {
        LeftBarWidget {
            buttons: vec![Default::default(), Default::default()],
        }
    }
}

impl LeftBarWidget {
}

pub enum LeftBarWidgetEvent {
}

impl ui::Component for LeftBarWidget {
    type EmittedEvent = LeftBarWidgetEvent;
    type ReceivedEvent = predefined::button::ButtonEvent;

    fn get_layout(&mut self) -> ui::Layout<predefined::button::ButtonEvent> {
        ui::Layout::VerticalBox(self.buttons.iter_mut().map(|b| b as &mut RawComponent<_>).collect())
    }

    fn handle_child_event(&mut self, child_id: usize, _: predefined::button::ButtonEvent)
                          -> Option<LeftBarWidgetEvent>
    {
        if child_id == 0 {

        } else if child_id == 1 {
        }

        None
    }
}
