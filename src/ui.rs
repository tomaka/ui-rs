use std::ops::{Deref, DerefMut};
use std::cmp;

use nalgebra::Vec2;
use shape::Shape;

use Component;
use RenderOutput;
use HoveredStatus;

/// The main struct of this library. Manages the whole user interface.
pub struct Ui<T> {
    main_component: T,
    shapes: Vec<Shape>,
    viewport: Vec2<u32>,
    mouse: Option<Vec2<u32>>,
}

/// Allows mutable access to the main component of the `Ui`.
pub struct UiMainComponentMutRef<'a, T: 'a> {
    ui: &'a mut Ui<T>,
}

impl<T> Ui<T> where T: Component {
    pub fn new(component: T, viewport: Vec2<u32>) -> Ui<T> {
        let mut ui = Ui {
            main_component: component,
            shapes: Vec::new(),
            viewport: viewport,
            mouse: None,
        };

        ui.update();
        ui
    }

    /// Sets the viewport of the user interface.
    pub fn set_viewport(&mut self, dimensions: Vec2<u32>) {
        self.viewport = dimensions;
        self.update();
    }

    /// Changes the position of the mouse over the UI.
    pub fn set_mouse_position(&mut self, position: Option<Vec2<u32>>) {
        self.mouse = position;
        self.update();
    }

    /// Gives a mutable access to the main component in order for you to modify it.
    pub fn get_mut_main_component(&mut self) -> UiMainComponentMutRef<T> {
        UiMainComponentMutRef { ui: self }
    }

    /// Returns the list of all the shapes that must be drawn as part of this UI.
    ///
    /// The list is sorted from bottom-z-to-top.
    pub fn draw(&self) -> &[Shape] {
        &self.shapes[]
    }

    fn update(&mut self) {
        let mouse = self.mouse.as_ref().map(|&coords| {
            Vec2::new(
                -1.0 + 2.0 * coords.x as f32 / self.viewport.x as f32,
                1.0 + -2.0 * coords.y as f32 / self.viewport.y as f32,
            )
        }).unwrap_or(Vec2::new(-1.0, -1.0));

        let (shapes, _, hierarchy, all) = process(self.main_component.render(), Vec2::new(0.0, 0.0),
                                                  mouse);
        self.shapes = shapes;

        for elem in all.iter() {
            let mut found = false;
            for (num, element) in hierarchy.iter().enumerate() {
                if unsafe {
                    let x: ::std::raw::TraitObject = ::std::mem::transmute(*element);
                    let y: ::std::raw::TraitObject = ::std::mem::transmute(*elem);
                    x.data == y.data && x.vtable == y.vtable
                }
                {
                    found = true;
                    element.set_hovered_status(if num == 0 {
                        HoveredStatus::Hovered
                    } else {
                        HoveredStatus::ChildHovered
                    });
                    break;
                }
            }

            if !found {
                elem.set_hovered_status(HoveredStatus::NotHovered);
            }
        }
    }
}

/// Returns a list of shapes to draw, the dimensions of the output, and the hierarchy of components
/// that are under the mouse cursor from inner to outter, and the hierarchy of all components.
fn process<'a>(output: RenderOutput<'a>, mut current_position: Vec2<f32>, mouse: Vec2<f32>)
               -> (Vec<Shape>, Vec2<f32>, Vec<&'a Component>, Vec<&'a Component>)
{
    match output {
        RenderOutput::HorizontalBox { children } => {
            let mut shapes = Vec::new();
            let mut max_height = 0.0;
            let mut main_hierarchy = None;
            let mut all = Vec::new();

            for child in children.into_iter() {
                let (child_shapes, child_dims, hierarchy, child_all) = process(child, current_position, mouse);
                all.extend(child_all.into_iter());
                if hierarchy.len() != 0 {
                    main_hierarchy = Some(hierarchy);
                }

                max_height = cmp::partial_max(max_height, child_dims.y).unwrap_or(max_height);
                shapes.extend(child_shapes.into_iter());
                current_position.x += child_dims.x;
            }

            (shapes, current_position, main_hierarchy.unwrap_or(vec![]), all)
        },

        RenderOutput::Component(child) => {
            let dimensions = child.get_dimensions();
            let bounding_box = child.get_bounding_box();
            let (shapes, alt_dims, mut hierarchy, mut all) = process(child.render(), current_position, mouse);
            let dimensions = dimensions.unwrap_or(alt_dims);

            let hierarchy = if let Some(bounding_box) = bounding_box {
                if mouse.x >= bounding_box.0.x + current_position.x &&
                    mouse.x <= bounding_box.1.x + current_position.x &&
                    mouse.y >= bounding_box.0.y + current_position.y &&
                    mouse.y <= bounding_box.1.y + current_position.y
                {
                    vec![child]
                } else {
                    vec![]
                }

            } else {
                if hierarchy.len() == 0 {
                    vec![]
                } else {
                    hierarchy.push(child);
                    hierarchy
                }
            };

            all.push(child);
            (shapes, dimensions, hierarchy, all)
        },

        RenderOutput::Shape(shape) => {
            let shape = shape.translate(current_position);
            (vec![shape], Vec2::new(0.0, 0.0), vec![], vec![])
        },
    }
}

impl<'a, T> Deref for UiMainComponentMutRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ui.main_component
    }
}

impl<'a, T> DerefMut for UiMainComponentMutRef<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ui.main_component
    }
}

#[unsafe_destructor]
impl<'a, T: 'a> Drop for UiMainComponentMutRef<'a, T> where T: Component {
    fn drop(&mut self) {
        self.ui.update();
    }
}
