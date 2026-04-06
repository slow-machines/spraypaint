//! The [`Colorize`] extension trait: add styling to any [`fmt::Display`](std::fmt::Display) value.
//!
//! Import [`Colorize`] to bring all methods into scope.
//!
//! # Example
//! ```
//! use spraypaint::Colorize;
//!
//! "Hello world".green().bold().paint();
//! "Warning".yellow().on_red().italic().paint();
//! ```

use std::fmt;

use crate::color::Color;
use crate::gradient::Gradient;
use crate::style::Style;
use crate::styled::Styled;

/// Extension trait that adds terminal styling methods to any [`fmt::Display`] type.
///
/// All color/style methods return a [`Styled<Self>`] (or [`Gradient`] for `.gradient()`),
/// which implements [`fmt::Display`] for composition and `.paint()` for direct output.
///
/// The blanket implementation `impl<T: Display> Colorize for T` means you never need to
/// implement this trait manually.
pub trait Colorize: fmt::Display + Sized {
    // ── Core ─────────────────────────────────────────────────────────────────

    /// Apply a complete [`Style`] to this value.
    fn style(self, s: Style) -> Styled<Self> {
        Styled::new(self, s)
    }

    // ── Foreground colors ─────────────────────────────────────────────────────

    /// Set the foreground color to black.
    fn black(self) -> Styled<Self> {
        self.style(Style::new().black())
    }
    /// Set the foreground color to red.
    fn red(self) -> Styled<Self> {
        self.style(Style::new().red())
    }
    /// Set the foreground color to green.
    fn green(self) -> Styled<Self> {
        self.style(Style::new().green())
    }
    /// Set the foreground color to yellow.
    fn yellow(self) -> Styled<Self> {
        self.style(Style::new().yellow())
    }
    /// Set the foreground color to blue.
    fn blue(self) -> Styled<Self> {
        self.style(Style::new().blue())
    }
    /// Set the foreground color to magenta.
    fn magenta(self) -> Styled<Self> {
        self.style(Style::new().magenta())
    }
    /// Set the foreground color to cyan.
    fn cyan(self) -> Styled<Self> {
        self.style(Style::new().cyan())
    }
    /// Set the foreground color to white.
    fn white(self) -> Styled<Self> {
        self.style(Style::new().white())
    }
    /// Set the foreground color to bright black (dark gray).
    fn bright_black(self) -> Styled<Self> {
        self.style(Style::new().bright_black())
    }
    /// Set the foreground color to bright red.
    fn bright_red(self) -> Styled<Self> {
        self.style(Style::new().bright_red())
    }
    /// Set the foreground color to bright green.
    fn bright_green(self) -> Styled<Self> {
        self.style(Style::new().bright_green())
    }
    /// Set the foreground color to bright yellow.
    fn bright_yellow(self) -> Styled<Self> {
        self.style(Style::new().bright_yellow())
    }
    /// Set the foreground color to bright blue.
    fn bright_blue(self) -> Styled<Self> {
        self.style(Style::new().bright_blue())
    }
    /// Set the foreground color to bright magenta.
    fn bright_magenta(self) -> Styled<Self> {
        self.style(Style::new().bright_magenta())
    }
    /// Set the foreground color to bright cyan.
    fn bright_cyan(self) -> Styled<Self> {
        self.style(Style::new().bright_cyan())
    }
    /// Set the foreground color to bright white.
    fn bright_white(self) -> Styled<Self> {
        self.style(Style::new().bright_white())
    }

    /// Set the foreground to a 24-bit RGB color.
    fn rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        self.style(Style::new().rgb(r, g, b))
    }

    /// Set the foreground from a CSS hex string (`#RRGGBB` or `#RGB`).
    fn hex(self, hex: &str) -> Styled<Self> {
        self.style(Style::new().hex(hex))
    }

    /// Set the foreground to an Xterm 256-color palette index (0–255).
    fn xterm(self, idx: u8) -> Styled<Self> {
        self.style(Style::new().fg(Color::Xterm(idx)))
    }

    // ── Background colors ─────────────────────────────────────────────────────

    /// Set the background color to black.
    fn on_black(self) -> Styled<Self> {
        self.style(Style::new().on_black())
    }
    /// Set the background color to red.
    fn on_red(self) -> Styled<Self> {
        self.style(Style::new().on_red())
    }
    /// Set the background color to green.
    fn on_green(self) -> Styled<Self> {
        self.style(Style::new().on_green())
    }
    /// Set the background color to yellow.
    fn on_yellow(self) -> Styled<Self> {
        self.style(Style::new().on_yellow())
    }
    /// Set the background color to blue.
    fn on_blue(self) -> Styled<Self> {
        self.style(Style::new().on_blue())
    }
    /// Set the background color to magenta.
    fn on_magenta(self) -> Styled<Self> {
        self.style(Style::new().on_magenta())
    }
    /// Set the background color to cyan.
    fn on_cyan(self) -> Styled<Self> {
        self.style(Style::new().on_cyan())
    }
    /// Set the background color to white.
    fn on_white(self) -> Styled<Self> {
        self.style(Style::new().on_white())
    }
    /// Set the background color to bright black (dark gray).
    fn on_bright_black(self) -> Styled<Self> {
        self.style(Style::new().on_bright_black())
    }
    /// Set the background color to bright red.
    fn on_bright_red(self) -> Styled<Self> {
        self.style(Style::new().on_bright_red())
    }
    /// Set the background color to bright green.
    fn on_bright_green(self) -> Styled<Self> {
        self.style(Style::new().on_bright_green())
    }
    /// Set the background color to bright yellow.
    fn on_bright_yellow(self) -> Styled<Self> {
        self.style(Style::new().on_bright_yellow())
    }
    /// Set the background color to bright blue.
    fn on_bright_blue(self) -> Styled<Self> {
        self.style(Style::new().on_bright_blue())
    }
    /// Set the background color to bright magenta.
    fn on_bright_magenta(self) -> Styled<Self> {
        self.style(Style::new().on_bright_magenta())
    }
    /// Set the background color to bright cyan.
    fn on_bright_cyan(self) -> Styled<Self> {
        self.style(Style::new().on_bright_cyan())
    }
    /// Set the background color to bright white.
    fn on_bright_white(self) -> Styled<Self> {
        self.style(Style::new().on_bright_white())
    }

    /// Set the background to a 24-bit RGB color.
    fn on_rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        self.style(Style::new().on_rgb(r, g, b))
    }

    /// Set the background from a CSS hex string (`#RRGGBB` or `#RGB`).
    fn on_hex(self, hex: &str) -> Styled<Self> {
        self.style(Style::new().on_hex(hex))
    }

    // ── Text attributes ───────────────────────────────────────────────────────

    /// Apply bold / increased intensity.
    fn bold(self) -> Styled<Self> {
        self.style(Style::new().bold())
    }
    /// Apply dim / faint / decreased intensity.
    fn dim(self) -> Styled<Self> {
        self.style(Style::new().dim())
    }
    /// Apply italic.
    fn italic(self) -> Styled<Self> {
        self.style(Style::new().italic())
    }
    /// Apply underline.
    fn underline(self) -> Styled<Self> {
        self.style(Style::new().underline())
    }
    /// Apply slow blink.
    fn blink(self) -> Styled<Self> {
        self.style(Style::new().blink())
    }
    /// Apply rapid blink.
    fn blink_fast(self) -> Styled<Self> {
        self.style(Style::new().blink_fast())
    }
    /// Apply reverse video (swap foreground and background).
    fn reverse(self) -> Styled<Self> {
        self.style(Style::new().reverse())
    }
    /// Apply hidden / concealed text.
    fn hidden(self) -> Styled<Self> {
        self.style(Style::new().hidden())
    }
    /// Apply strikethrough / crossed-out text.
    fn strikethrough(self) -> Styled<Self> {
        self.style(Style::new().strikethrough())
    }

    // ── Gradient ─────────────────────────────────────────────────────────────

    /// Apply a two-stop gradient across the characters of this value's string representation.
    fn gradient(self, from: Color, to: Color) -> Gradient {
        Gradient::new(self.to_string(), from, to)
    }

    /// Apply a multi-stop gradient (must have at least two stops).
    ///
    /// # Panics
    /// Panics if `stops` has fewer than two elements.
    fn gradient_multi(self, stops: Vec<Color>) -> Gradient {
        Gradient::multi_stop(self.to_string(), stops)
    }
}

/// Blanket implementation: any `T: Display` can be colorized.
impl<T: fmt::Display> Colorize for T {}

// ── `Styled<T>` flat chaining ─────────────────────────────────────────────────
//
// `Styled<T>: Display`, so the blanket impl above applies, but calling
// `.red()` via the trait would return `Styled<Styled<T>>` (nested wrappers).
// These inherent methods mutate `self.style` in-place and return `Self`,
// keeping the chain flat and allocation-free.

impl<T: fmt::Display> Styled<T> {
    /// Override the foreground color with red.
    pub fn red(mut self) -> Self {
        self.style = self.style.red();
        self
    }
    /// Override the foreground color with green.
    pub fn green(mut self) -> Self {
        self.style = self.style.green();
        self
    }
    /// Override the foreground color with yellow.
    pub fn yellow(mut self) -> Self {
        self.style = self.style.yellow();
        self
    }
    /// Override the foreground color with blue.
    pub fn blue(mut self) -> Self {
        self.style = self.style.blue();
        self
    }
    /// Override the foreground color with magenta.
    pub fn magenta(mut self) -> Self {
        self.style = self.style.magenta();
        self
    }
    /// Override the foreground color with cyan.
    pub fn cyan(mut self) -> Self {
        self.style = self.style.cyan();
        self
    }
    /// Override the foreground color with white.
    pub fn white(mut self) -> Self {
        self.style = self.style.white();
        self
    }
    /// Override the foreground color with black.
    pub fn black(mut self) -> Self {
        self.style = self.style.black();
        self
    }
    /// Override the foreground color with bright red.
    pub fn bright_red(mut self) -> Self {
        self.style = self.style.bright_red();
        self
    }
    /// Override the foreground color with bright green.
    pub fn bright_green(mut self) -> Self {
        self.style = self.style.bright_green();
        self
    }
    /// Override the foreground color with bright yellow.
    pub fn bright_yellow(mut self) -> Self {
        self.style = self.style.bright_yellow();
        self
    }
    /// Override the foreground color with bright blue.
    pub fn bright_blue(mut self) -> Self {
        self.style = self.style.bright_blue();
        self
    }
    /// Override the foreground color with bright magenta.
    pub fn bright_magenta(mut self) -> Self {
        self.style = self.style.bright_magenta();
        self
    }
    /// Override the foreground color with bright cyan.
    pub fn bright_cyan(mut self) -> Self {
        self.style = self.style.bright_cyan();
        self
    }
    /// Override the foreground color with bright white.
    pub fn bright_white(mut self) -> Self {
        self.style = self.style.bright_white();
        self
    }
    /// Override the foreground color with bright black (dark gray).
    pub fn bright_black(mut self) -> Self {
        self.style = self.style.bright_black();
        self
    }

    /// Set the background color to black.
    pub fn on_black(mut self) -> Self {
        self.style = self.style.on_black();
        self
    }
    /// Set the background color to red.
    pub fn on_red(mut self) -> Self {
        self.style = self.style.on_red();
        self
    }
    /// Set the background color to green.
    pub fn on_green(mut self) -> Self {
        self.style = self.style.on_green();
        self
    }
    /// Set the background color to yellow.
    pub fn on_yellow(mut self) -> Self {
        self.style = self.style.on_yellow();
        self
    }
    /// Set the background color to blue.
    pub fn on_blue(mut self) -> Self {
        self.style = self.style.on_blue();
        self
    }
    /// Set the background color to magenta.
    pub fn on_magenta(mut self) -> Self {
        self.style = self.style.on_magenta();
        self
    }
    /// Set the background color to cyan.
    pub fn on_cyan(mut self) -> Self {
        self.style = self.style.on_cyan();
        self
    }
    /// Set the background color to white.
    pub fn on_white(mut self) -> Self {
        self.style = self.style.on_white();
        self
    }
    /// Set the background color to bright black (dark gray).
    pub fn on_bright_black(mut self) -> Self {
        self.style = self.style.on_bright_black();
        self
    }
    /// Set the background color to bright red.
    pub fn on_bright_red(mut self) -> Self {
        self.style = self.style.on_bright_red();
        self
    }
    /// Set the background color to bright green.
    pub fn on_bright_green(mut self) -> Self {
        self.style = self.style.on_bright_green();
        self
    }
    /// Set the background color to bright yellow.
    pub fn on_bright_yellow(mut self) -> Self {
        self.style = self.style.on_bright_yellow();
        self
    }
    /// Set the background color to bright blue.
    pub fn on_bright_blue(mut self) -> Self {
        self.style = self.style.on_bright_blue();
        self
    }
    /// Set the background color to bright magenta.
    pub fn on_bright_magenta(mut self) -> Self {
        self.style = self.style.on_bright_magenta();
        self
    }
    /// Set the background color to bright cyan.
    pub fn on_bright_cyan(mut self) -> Self {
        self.style = self.style.on_bright_cyan();
        self
    }
    /// Set the background color to bright white.
    pub fn on_bright_white(mut self) -> Self {
        self.style = self.style.on_bright_white();
        self
    }

    /// Set the foreground to an RGB color.
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.rgb(r, g, b);
        self
    }
    /// Set the background to an RGB color.
    pub fn on_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.on_rgb(r, g, b);
        self
    }
    /// Set the foreground from a CSS hex string.
    pub fn hex(mut self, h: &str) -> Self {
        self.style = self.style.hex(h);
        self
    }
    /// Set the background from a CSS hex string.
    pub fn on_hex(mut self, h: &str) -> Self {
        self.style = self.style.on_hex(h);
        self
    }

    /// Apply bold.
    pub fn bold(mut self) -> Self {
        self.style = self.style.bold();
        self
    }
    /// Apply dim.
    pub fn dim(mut self) -> Self {
        self.style = self.style.dim();
        self
    }
    /// Apply italic.
    pub fn italic(mut self) -> Self {
        self.style = self.style.italic();
        self
    }
    /// Apply underline.
    pub fn underline(mut self) -> Self {
        self.style = self.style.underline();
        self
    }
    /// Apply slow blink.
    pub fn blink(mut self) -> Self {
        self.style = self.style.blink();
        self
    }
    /// Apply rapid blink.
    pub fn blink_fast(mut self) -> Self {
        self.style = self.style.blink_fast();
        self
    }
    /// Apply reverse video.
    pub fn reverse(mut self) -> Self {
        self.style = self.style.reverse();
        self
    }
    /// Apply hidden / concealed text.
    pub fn hidden(mut self) -> Self {
        self.style = self.style.hidden();
        self
    }
    /// Apply strikethrough.
    pub fn strikethrough(mut self) -> Self {
        self.style = self.style.strikethrough();
        self
    }
}
