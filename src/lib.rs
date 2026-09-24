#[cfg(debug_assertions)]
mod debug;
mod formatter;
mod node;
mod rules;
mod text;

pub use formatter::{Error, format_code};
