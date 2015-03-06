#![feature(unsafe_destructor)]
#![warn(missing_docs)]
#![unstable]

extern crate nalgebra;

use std::default::Default;

pub use nalgebra::Vec2;
pub use shape::Shape;
pub use easy_component::{Component, Layout};
pub use ui::{Ui, UiMainComponentMutRef};

pub mod component;
pub mod predefined;

mod easy_component;
mod shape;
mod ui;
