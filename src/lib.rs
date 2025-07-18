#[cfg(debug_assertions)]
mod debug;
mod error;
mod formatter;
mod node;
mod rules;
mod text;

pub use error::Error;
pub use formatter::format_code;
