use std::default::Default;

use ui;
use ui::predefined;
use ui::component::RawComponent;

use left_bar;

pub struct MainUi {
    left_bar: left_bar::LeftBarWidget,
}

impl Default for MainUi {
    fn default() -> MainUi {
        MainUi {
            left_bar: Default::default(),
        }
    }
}

impl MainUi {
}

pub enum MainUiEvent {
}

impl ui::Component for MainUi {
    type EmittedEvent = MainUiEvent;
    type ReceivedEvent = left_bar::LeftBarWidgetEvent;

    fn get_layout(&mut self) -> ui::Layout {
        ui::Layout::PositionnedChildren(vec![
            ui::PositionnedChild {
                child: &mut self.left_bar as &mut RawComponent,
                x: -0.5,
                y: 0.0,
            }
        ])
    }

    fn handle_child_event(&mut self, child_id: usize, _: &left_bar::LeftBarWidgetEvent)
                          -> Option<MainUiEvent>
    {
        None
    }
}
