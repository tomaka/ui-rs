use std::any::Any;
use nalgebra::Vec2;

use component::RawComponent;
use Shape;

pub trait Component: Send + Sync + 'static {
    /// What events type this component produces.
    type EmittedEvent: 'static = ();

    /// What events type this component expects to receive.
    type ReceivedEvent: 'static = ();

    /// 
    fn get_layout(&mut self) -> Layout;

    /// A child has produced an event.
    fn handle_child_event(&mut self, child_id: usize, &Self::ReceivedEvent)
                          -> Option<Self::EmittedEvent>
    {
        None
    }

    /// Sets whether this component is hovered by the mouse or not.
    ///
    /// The default action is not to do anything.
    fn set_hovered_status(&self, HoveredStatus) -> Option<Self::EmittedEvent> { None }

    /// Returns the dimensions of the component. If returns `None`, the dimensions are
    /// automatically calculated using what `render` returns.
    ///
    /// If `render` only returns shapes, then your component will have a dimension of `(0.0, 0.0)`.
    ///
    /// The dimensions are used when calculating layouts.
    fn get_dimensions(&self) -> Option<Vec2<f32>> {
        None
    }

    /// Returns the bounding box of the component. If returns `None`, the bounding box corresponds
    /// to the bounding boxes of what `render` returns. The default behavior of this function is
    /// to return `None`, and it is what you should usually do.
    ///
    /// The bounding box is used to determine whether the cursor is hovering the component.
    fn get_bounding_box(&self) -> Option<(Vec2<f32>, Vec2<f32>)> {
        None
    }
}

pub enum Layout<'a> {
    SingleChild(&'a mut RawComponent),

    HorizontalBox(Vec<&'a mut RawComponent>),

    VerticalBox(Vec<&'a mut RawComponent>),

    PositionnedChildren(Vec<PositionnedChild<'a>>),
}

pub struct PositionnedChild<'a> {
    pub child: &'a mut RawComponent,
    pub x: f32,
    pub y: f32,
}

impl<T> RawComponent for T where T: Component {
    fn render(&mut self) -> Vec<Shape> {
        match self.get_layout() {
            Layout::SingleChild(child) => {
                child.render()
            },

            Layout::HorizontalBox(children) => {
                let mut result = Vec::new();
                let mut x = 0.0;

                for child in children {
                    result.extend(child.render().into_iter().map(|s| s.translate(Vec2::new(x, 0.0))));
                    x += child.get_width();
                }

                result
            },

            Layout::VerticalBox(children) => {
                let mut result = Vec::new();
                let mut y = 0.0;

                for child in children {
                    result.extend(child.render().into_iter().map(|s| s.translate(Vec2::new(0.0, y))));
                    y += child.get_height();
                }

                result
            },

            Layout::PositionnedChildren(children) => {
                let mut result = Vec::new();
                for child in children {
                    result.extend(child.child.render().into_iter().map(|s| s.translate(Vec2::new(child.x, child.y))));
                }
                result
            },
        }
    }

    fn set_mouse_status(&mut self, position: Option<Vec2<f32>>, pressed: bool) -> Vec<Box<Any>> {
        let events = match self.get_layout() {
            Layout::SingleChild(child) => {
                child.set_mouse_status(position, pressed).into_iter().map(|ev| (0usize, ev)).collect::<Vec<_>>()
            },

            Layout::HorizontalBox(children) => {
                let mut events = Vec::with_capacity(0);

                if let Some(position) = position {
                    let mut position = position;
                    let mut found = false;

                    for (child_id, child) in children.into_iter().enumerate() {
                        if found {
                            events.extend(child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                            continue;
                        }

                        if child.hit_test(position) {
                            events.extend(child.set_mouse_status(Some(position), pressed).into_iter().map(|ev| (child_id, ev)));
                            found = true;
                            continue;
                        } else {
                            events.extend(child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                        }

                        position.x -= child.get_width();
                    }

                } else {
                    for (child_id, child) in children.into_iter().enumerate() {
                        events.extend(child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                    }
                }

                events
            },

            Layout::VerticalBox(children) => {
                let mut events = Vec::with_capacity(0);

                if let Some(position) = position {
                    let mut position = position;
                    let mut found = false;

                    for (child_id, child) in children.into_iter().enumerate() {
                        if found {
                            events.extend(child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                            continue;
                        }

                        if child.hit_test(position) {
                            events.extend(child.set_mouse_status(Some(position), pressed).into_iter().map(|ev| (child_id, ev)));
                            found = true;
                            continue;
                        } else {
                            events.extend(child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                        }

                        position.y -= child.get_height();
                    }

                } else {
                    for (child_id, child) in children.into_iter().enumerate() {
                        events.extend(child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                    }
                }

                events
            },

            Layout::PositionnedChildren(children) => {
                let mut events = Vec::with_capacity(0);


                if let Some(position) = position {
                    let mut position = position;
                    let mut found = false;

                    for (child_id, child) in children.into_iter().enumerate() {
                        let position = {
                            let mut p = position;
                            p.x -= child.x;
                            p.y -= child.y;
                            p
                        };

                        if found {
                            events.extend(child.child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                            continue;
                        }

                        if child.child.hit_test(position) {
                            events.extend(child.child.set_mouse_status(Some(position), pressed).into_iter().map(|ev| (child_id, ev)));
                            found = true;
                            continue;
                        } else {
                            events.extend(child.child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                        }
                    }

                } else {
                    for (child_id, child) in children.into_iter().enumerate() {
                        events.extend(child.child.set_mouse_status(None, pressed).into_iter().map(|ev| (child_id, ev)));
                    }
                }

                events
            },
        };

        events.into_iter().filter_map(|(id, ev)| {
            self.handle_raw_child_event(id, ev)
        }).collect()
    }

    fn hit_test(&mut self, position: Vec2<f32>) -> bool {
        match self.get_layout() {
            Layout::SingleChild(child) => {
                child.hit_test(position)
            },

            Layout::HorizontalBox(children) => {
                let mut position = position;

                for child in children {
                    if child.hit_test(position) {
                        return true;
                    }

                    position.x -= child.get_width();
                }

                false
            },

            Layout::VerticalBox(children) => {
                let mut position = position;

                for child in children {
                    if child.hit_test(position) {
                        return true;
                    }

                    position.y -= child.get_height();
                }

                false
            },

            Layout::PositionnedChildren(children) => {
                for child in children {
                    let mut position = position;
                    position.x -= child.x;
                    position.y -= child.y;

                    if child.child.hit_test(position) {
                        return true;
                    }
                }

                false
            },
        }
    }

    fn get_width(&mut self) -> f32 {
        match self.get_layout() {
            Layout::SingleChild(child) => {
                child.get_width()
            },

            Layout::HorizontalBox(children) => {
                use std::iter::AdditiveIterator;
                children.into_iter().map(|c| c.get_width()).sum()
            },

            Layout::VerticalBox(children) => {
                let mut max = 0.0;
                for child in children {
                    let width = child.get_width();
                    if width > max {
                        max = width;
                    }
                }
                max
            },

            Layout::PositionnedChildren(_) => {
                0.0
            },
        }
    }

    fn get_height(&mut self) -> f32 {
        match self.get_layout() {
            Layout::SingleChild(child) => {
                child.get_height()
            },

            Layout::HorizontalBox(children) => {
                let mut max = 0.0;
                for child in children {
                    let width = child.get_height();
                    if width > max {
                        max = width;
                    }
                }
                max
            },

            Layout::VerticalBox(children) => {
                use std::iter::AdditiveIterator;
                children.into_iter().map(|c| c.get_height()).sum()
            },

            Layout::PositionnedChildren(_) => {
                0.0
            },
        }
    }

    fn handle_raw_child_event(&mut self, child_id: usize, event: Box<Any>) -> Option<Box<Any>> {
        let ev = if let Some(event) = event.downcast_ref() {
            let event: &<Self as Component>::ReceivedEvent = event;
            self.handle_child_event(child_id, event)
        } else {
            panic!("Mismatch between emitted and received events")
        };

        ev.map(|ev| Box::new(ev) as Box<Any>)
    }
}

/// State of a component in regards to the mouse position.
#[derive(Debug, Clone, Copy)]
pub enum HoveredStatus {
    Hovered,
    ChildHovered,
    NotHovered,
}
