#![warn(missing_docs)]

extern crate nalgebra;

use std::default::Default;

pub use nalgebra::Vec2;
pub use shape::Shape;
pub use easy_component::{Component, Layout, PositionnedChild};
pub use ui::{Ui, UiMainComponentMutRef};

pub mod component;
pub mod predefined;

mod easy_component;
mod shape;
mod ui;
