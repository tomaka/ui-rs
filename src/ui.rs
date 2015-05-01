use std::ops::{Deref, DerefMut};
use std::cmp;
use std::marker::PhantomData;

use nalgebra::Vec2;
use shape::Shape;

use component::RawComponent;

/// The main struct of this library. Manages the whole user interface.
pub struct Ui<T, E> {
    main_component: T,
    marker: PhantomData<E>,
    shapes: Vec<Shape>,
    viewport: Vec2<u32>,
    mouse: Option<Vec2<u32>>,
    mouse_pressed: bool,
}

/// Allows mutable access to the main component of the `Ui`.
pub struct UiMainComponentMutRef<'a, T: 'a, E: 'a> where T: RawComponent<E> {
    ui: &'a mut Ui<T, E>,
}

impl<T, E> Ui<T, E> where T: RawComponent<E> {
    pub fn new(component: T, viewport: Vec2<u32>) -> Ui<T, E> {
        let mut ui = Ui {
            main_component: component,
            marker: PhantomData,
            shapes: Vec::new(),
            viewport: viewport,
            mouse: None,
            mouse_pressed: false,
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

    /// Sets whether the mouse is pressed.
    pub fn set_mouse_pressed(&mut self, pressed: bool) {
        self.mouse_pressed = pressed;
        self.update();
    }

    /// Gives a mutable access to the main component in order for you to modify it.
    pub fn get_mut_main_component(&mut self) -> UiMainComponentMutRef<T, E> {
        UiMainComponentMutRef { ui: self }
    }

    /// Returns the list of all the shapes that must be drawn as part of this UI.
    ///
    /// The list is sorted from bottom-z-to-top.
    pub fn draw(&self) -> &[Shape] {
        &self.shapes
    }

    fn update(&mut self) {
        let mouse = self.mouse.as_ref().map(|&coords| {
            Vec2::new(
                -1.0 + 2.0 * coords.x as f32 / self.viewport.x as f32,
                1.0 + -2.0 * coords.y as f32 / self.viewport.y as f32,
            )
        }).unwrap_or(Vec2::new(-1.0, -1.0));

        //self.main_component.set_viewport      // TODO: 
        self.main_component.set_mouse_status(Some(mouse), self.mouse_pressed);
        self.shapes = self.main_component.render();
    }
}

impl<'a, T, E> Deref for UiMainComponentMutRef<'a, T, E> where T: RawComponent<E> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ui.main_component
    }
}

impl<'a, T, E> DerefMut for UiMainComponentMutRef<'a, T, E> where T: RawComponent<E> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ui.main_component
    }
}

impl<'a, T: 'a, E> Drop for UiMainComponentMutRef<'a, T, E> where T: RawComponent<E> {
    fn drop(&mut self) {
        self.ui.update();
    }
}
