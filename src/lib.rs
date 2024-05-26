#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod data;
pub use app::ViewerApp;
pub use data::dataset;
