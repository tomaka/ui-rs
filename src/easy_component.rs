use nalgebra::Vec2;

use component::RawComponent;
use Shape;

pub trait Component: Send + Sync + 'static {
    /// What events type this component produces.
    type EmittedEvent: 'static = ();

    /// What events type this component expects to receive.
    type ReceivedEvent: 'static = ();

    /// 
    fn get_layout(&self) -> Layout;

    /// 
    fn get_mut_layout(&mut self) -> MutLayout;

    /// A child has produced an event.
    fn handle_child_event(&mut self, Self::ReceivedEvent) {
    }

    /// Sets whether this component is hovered by the mouse or not.
    ///
    /// The default action is not to do anything.
    fn set_hovered_status(&self, HoveredStatus) { }

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
    SingleChild(&'a RawComponent),

    HorizontalBox(Vec<&'a RawComponent>),
}

pub enum MutLayout<'a> {
    SingleChild(&'a mut RawComponent),

    HorizontalBox(Vec<&'a mut RawComponent>),
}

impl<T> RawComponent for T where T: Component {
    fn render(&self) -> Vec<Shape> {
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
        }
    }

    fn set_mouse_position(&mut self, position: Option<Vec2<f32>>) {
        match self.get_mut_layout() {
            MutLayout::SingleChild(child) => {
                child.set_mouse_position(position)
            },
            MutLayout::HorizontalBox(children) => {
                if let Some(position) = position {
                    let mut position = position;
                    let mut found = false;

                    for child in children {
                        if found {
                            child.set_mouse_position(None);
                            continue;
                        }

                        if child.hit_test(position) {
                            child.set_mouse_position(Some(position));
                            found = true;
                            continue;
                        } else {
                            child.set_mouse_position(None);
                        }

                        position.x -= child.get_width();
                    }

                } else {
                    for child in children {
                        child.set_mouse_position(None);
                    }
                }
            },
        }
    }

    fn hit_test(&self, position: Vec2<f32>) -> bool {
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
        }
    }

    fn get_width(&self) -> f32 {
        unimplemented!()
    }
}

/// State of a component in regards to the mouse position.
#[derive(Debug, Clone, Copy)]
pub enum HoveredStatus {
    Hovered,
    ChildHovered,
    NotHovered,
}
