//! # spraypaint
//!
//! Terminal string styling for Rust -- inspired by [Chalk](https://github.com/chalk/chalk).
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use spraypaint::{paint, Colorize};
//!
//! // Primary API: paint! prints directly (like println!)
//! paint!("{red.bold Error:} something went wrong");
//! paint!("Hello {green.italic world}!");
//! paint!(inline, "{yellow Loading...}");   // no newline
//! paint!(stderr, "{red.bold FATAL:} disk full");
//!
//! // Extension trait: chain styles, then call .paint()
//! "Warning".yellow().on_red().italic().paint();
//!
//! // Compose styled values for format strings / loggers
//! let msg = "critical".red().bold();
//! eprintln!("status: {msg}");
//! ```
//!
//! ## Color Levels
//!
//! spraypaint auto-detects terminal color support via environment variables:
//! - `NO_COLOR` → disable all color
//! - `FORCE_COLOR=3` → force truecolor
//! - `COLORTERM=truecolor` → enable RGB
//!
//! Override programmatically:
//! ```rust,no_run
//! use spraypaint::detect::{set_color_level, ColorLevel};
//! set_color_level(ColorLevel::TrueColor);
//! ```
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub(crate) mod ansi;
pub(crate) mod styled;

pub mod color;
pub mod detect;
pub mod ext;
pub mod gradient;
pub mod style;
pub mod utils;

pub use color::Color;
pub use detect::{color_level, set_color_level, ColorLevel};
pub use ext::Colorize;
pub use gradient::Gradient;
pub use style::{Attrs, Style};
pub use styled::Styled;
pub use utils::strip_ansi;

// Macros from the proc-macro crate.
pub use spraypaint_macros::paint;
pub use spraypaint_macros::styled;

/// Convenience re-export: `use spraypaint::prelude::*` brings the most common items into scope.
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::ext::Colorize;
    pub use crate::paint;
    pub use crate::style::Style;
    pub use crate::styled;
}
