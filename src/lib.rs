#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod data;
mod extent_panel;
mod features;
mod fields;
mod layer_panel;
mod srs;
pub use app::ViewerApp;
pub use data::dataset;
