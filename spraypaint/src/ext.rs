//! [`Colorize`] extension trait — adds styling methods to any [`fmt::Display`] type.
//!
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

/// Adds terminal styling to any [`fmt::Display`] type via a blanket impl.
///
/// Returns [`Styled<Self>`] (or [`Gradient`] for `.gradient()`), both of which
/// implement `Display` and expose `.paint()` for direct output.
pub trait Colorize: fmt::Display + Sized {
    /// Wrap with a [`Style`].
    fn style(self, s: Style) -> Styled<Self> {
        Styled::new(self, s)
    }

    // Foreground colors

    /// Black foreground.
    fn black(self) -> Styled<Self> {
        self.style(Style::new().black())
    }
    /// Red foreground.
    fn red(self) -> Styled<Self> {
        self.style(Style::new().red())
    }
    /// Green foreground.
    fn green(self) -> Styled<Self> {
        self.style(Style::new().green())
    }
    /// Yellow foreground.
    fn yellow(self) -> Styled<Self> {
        self.style(Style::new().yellow())
    }
    /// Blue foreground.
    fn blue(self) -> Styled<Self> {
        self.style(Style::new().blue())
    }
    /// Magenta foreground.
    fn magenta(self) -> Styled<Self> {
        self.style(Style::new().magenta())
    }
    /// Cyan foreground.
    fn cyan(self) -> Styled<Self> {
        self.style(Style::new().cyan())
    }
    /// White foreground.
    fn white(self) -> Styled<Self> {
        self.style(Style::new().white())
    }
    /// Bright black (dark gray) foreground.
    fn bright_black(self) -> Styled<Self> {
        self.style(Style::new().bright_black())
    }
    /// Bright red foreground.
    fn bright_red(self) -> Styled<Self> {
        self.style(Style::new().bright_red())
    }
    /// Bright green foreground.
    fn bright_green(self) -> Styled<Self> {
        self.style(Style::new().bright_green())
    }
    /// Bright yellow foreground.
    fn bright_yellow(self) -> Styled<Self> {
        self.style(Style::new().bright_yellow())
    }
    /// Bright blue foreground.
    fn bright_blue(self) -> Styled<Self> {
        self.style(Style::new().bright_blue())
    }
    /// Bright magenta foreground.
    fn bright_magenta(self) -> Styled<Self> {
        self.style(Style::new().bright_magenta())
    }
    /// Bright cyan foreground.
    fn bright_cyan(self) -> Styled<Self> {
        self.style(Style::new().bright_cyan())
    }
    /// Bright white foreground.
    fn bright_white(self) -> Styled<Self> {
        self.style(Style::new().bright_white())
    }

    /// 24-bit RGB foreground.
    fn rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        self.style(Style::new().rgb(r, g, b))
    }

    /// Foreground from hex string (`#RRGGBB` or `#RGB`).
    fn hex(self, hex: &str) -> Styled<Self> {
        self.style(Style::new().hex(hex))
    }

    /// Xterm 256-color foreground (0–255).
    fn xterm(self, idx: u8) -> Styled<Self> {
        self.style(Style::new().fg(Color::Xterm(idx)))
    }

    // Background colors

    /// Black background.
    fn on_black(self) -> Styled<Self> {
        self.style(Style::new().on_black())
    }
    /// Red background.
    fn on_red(self) -> Styled<Self> {
        self.style(Style::new().on_red())
    }
    /// Green background.
    fn on_green(self) -> Styled<Self> {
        self.style(Style::new().on_green())
    }
    /// Yellow background.
    fn on_yellow(self) -> Styled<Self> {
        self.style(Style::new().on_yellow())
    }
    /// Blue background.
    fn on_blue(self) -> Styled<Self> {
        self.style(Style::new().on_blue())
    }
    /// Magenta background.
    fn on_magenta(self) -> Styled<Self> {
        self.style(Style::new().on_magenta())
    }
    /// Cyan background.
    fn on_cyan(self) -> Styled<Self> {
        self.style(Style::new().on_cyan())
    }
    /// White background.
    fn on_white(self) -> Styled<Self> {
        self.style(Style::new().on_white())
    }
    /// Bright black (dark gray) background.
    fn on_bright_black(self) -> Styled<Self> {
        self.style(Style::new().on_bright_black())
    }
    /// Bright red background.
    fn on_bright_red(self) -> Styled<Self> {
        self.style(Style::new().on_bright_red())
    }
    /// Bright green background.
    fn on_bright_green(self) -> Styled<Self> {
        self.style(Style::new().on_bright_green())
    }
    /// Bright yellow background.
    fn on_bright_yellow(self) -> Styled<Self> {
        self.style(Style::new().on_bright_yellow())
    }
    /// Bright blue background.
    fn on_bright_blue(self) -> Styled<Self> {
        self.style(Style::new().on_bright_blue())
    }
    /// Bright magenta background.
    fn on_bright_magenta(self) -> Styled<Self> {
        self.style(Style::new().on_bright_magenta())
    }
    /// Bright cyan background.
    fn on_bright_cyan(self) -> Styled<Self> {
        self.style(Style::new().on_bright_cyan())
    }
    /// Bright white background.
    fn on_bright_white(self) -> Styled<Self> {
        self.style(Style::new().on_bright_white())
    }

    /// 24-bit RGB background.
    fn on_rgb(self, r: u8, g: u8, b: u8) -> Styled<Self> {
        self.style(Style::new().on_rgb(r, g, b))
    }

    /// Background from hex string (`#RRGGBB` or `#RGB`).
    fn on_hex(self, hex: &str) -> Styled<Self> {
        self.style(Style::new().on_hex(hex))
    }

    // Text attributes

    /// Bold.
    fn bold(self) -> Styled<Self> {
        self.style(Style::new().bold())
    }
    /// Dim / faint.
    fn dim(self) -> Styled<Self> {
        self.style(Style::new().dim())
    }
    /// Italic.
    fn italic(self) -> Styled<Self> {
        self.style(Style::new().italic())
    }
    /// Underline.
    fn underline(self) -> Styled<Self> {
        self.style(Style::new().underline())
    }
    /// Slow blink.
    fn blink(self) -> Styled<Self> {
        self.style(Style::new().blink())
    }
    /// Rapid blink.
    fn blink_fast(self) -> Styled<Self> {
        self.style(Style::new().blink_fast())
    }
    /// Reverse video.
    fn reverse(self) -> Styled<Self> {
        self.style(Style::new().reverse())
    }
    /// Hidden / concealed.
    fn hidden(self) -> Styled<Self> {
        self.style(Style::new().hidden())
    }
    /// Strikethrough.
    fn strikethrough(self) -> Styled<Self> {
        self.style(Style::new().strikethrough())
    }

    // Gradient

    /// Two-stop color gradient across the displayed characters.
    fn gradient(self, from: Color, to: Color) -> Gradient {
        Gradient::new(self.to_string(), from, to)
    }

    /// Multi-stop gradient. Panics if `stops` has fewer than 2 elements.
    fn gradient_multi(self, stops: Vec<Color>) -> Gradient {
        Gradient::multi_stop(self.to_string(), stops)
    }
}

impl<T: fmt::Display> Colorize for T {}

// Inherent methods on `Styled<T>` so chaining mutates in-place instead of
// nesting `Styled<Styled<T>>` through the blanket Colorize impl.

impl<T: fmt::Display> Styled<T> {
    /// Red foreground.
    pub fn red(mut self) -> Self {
        self.style = self.style.red();
        self
    }
    /// Green foreground.
    pub fn green(mut self) -> Self {
        self.style = self.style.green();
        self
    }
    /// Yellow foreground.
    pub fn yellow(mut self) -> Self {
        self.style = self.style.yellow();
        self
    }
    /// Blue foreground.
    pub fn blue(mut self) -> Self {
        self.style = self.style.blue();
        self
    }
    /// Magenta foreground.
    pub fn magenta(mut self) -> Self {
        self.style = self.style.magenta();
        self
    }
    /// Cyan foreground.
    pub fn cyan(mut self) -> Self {
        self.style = self.style.cyan();
        self
    }
    /// White foreground.
    pub fn white(mut self) -> Self {
        self.style = self.style.white();
        self
    }
    /// Black foreground.
    pub fn black(mut self) -> Self {
        self.style = self.style.black();
        self
    }
    /// Bright red foreground.
    pub fn bright_red(mut self) -> Self {
        self.style = self.style.bright_red();
        self
    }
    /// Bright green foreground.
    pub fn bright_green(mut self) -> Self {
        self.style = self.style.bright_green();
        self
    }
    /// Bright yellow foreground.
    pub fn bright_yellow(mut self) -> Self {
        self.style = self.style.bright_yellow();
        self
    }
    /// Bright blue foreground.
    pub fn bright_blue(mut self) -> Self {
        self.style = self.style.bright_blue();
        self
    }
    /// Bright magenta foreground.
    pub fn bright_magenta(mut self) -> Self {
        self.style = self.style.bright_magenta();
        self
    }
    /// Bright cyan foreground.
    pub fn bright_cyan(mut self) -> Self {
        self.style = self.style.bright_cyan();
        self
    }
    /// Bright white foreground.
    pub fn bright_white(mut self) -> Self {
        self.style = self.style.bright_white();
        self
    }
    /// Bright black (dark gray) foreground.
    pub fn bright_black(mut self) -> Self {
        self.style = self.style.bright_black();
        self
    }

    /// Black background.
    pub fn on_black(mut self) -> Self {
        self.style = self.style.on_black();
        self
    }
    /// Red background.
    pub fn on_red(mut self) -> Self {
        self.style = self.style.on_red();
        self
    }
    /// Green background.
    pub fn on_green(mut self) -> Self {
        self.style = self.style.on_green();
        self
    }
    /// Yellow background.
    pub fn on_yellow(mut self) -> Self {
        self.style = self.style.on_yellow();
        self
    }
    /// Blue background.
    pub fn on_blue(mut self) -> Self {
        self.style = self.style.on_blue();
        self
    }
    /// Magenta background.
    pub fn on_magenta(mut self) -> Self {
        self.style = self.style.on_magenta();
        self
    }
    /// Cyan background.
    pub fn on_cyan(mut self) -> Self {
        self.style = self.style.on_cyan();
        self
    }
    /// White background.
    pub fn on_white(mut self) -> Self {
        self.style = self.style.on_white();
        self
    }
    /// Bright black (dark gray) background.
    pub fn on_bright_black(mut self) -> Self {
        self.style = self.style.on_bright_black();
        self
    }
    /// Bright red background.
    pub fn on_bright_red(mut self) -> Self {
        self.style = self.style.on_bright_red();
        self
    }
    /// Bright green background.
    pub fn on_bright_green(mut self) -> Self {
        self.style = self.style.on_bright_green();
        self
    }
    /// Bright yellow background.
    pub fn on_bright_yellow(mut self) -> Self {
        self.style = self.style.on_bright_yellow();
        self
    }
    /// Bright blue background.
    pub fn on_bright_blue(mut self) -> Self {
        self.style = self.style.on_bright_blue();
        self
    }
    /// Bright magenta background.
    pub fn on_bright_magenta(mut self) -> Self {
        self.style = self.style.on_bright_magenta();
        self
    }
    /// Bright cyan background.
    pub fn on_bright_cyan(mut self) -> Self {
        self.style = self.style.on_bright_cyan();
        self
    }
    /// Bright white background.
    pub fn on_bright_white(mut self) -> Self {
        self.style = self.style.on_bright_white();
        self
    }

    /// RGB foreground.
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.rgb(r, g, b);
        self
    }
    /// RGB background.
    pub fn on_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style = self.style.on_rgb(r, g, b);
        self
    }
    /// Hex foreground.
    pub fn hex(mut self, h: &str) -> Self {
        self.style = self.style.hex(h);
        self
    }
    /// Hex background.
    pub fn on_hex(mut self, h: &str) -> Self {
        self.style = self.style.on_hex(h);
        self
    }

    /// Bold.
    pub fn bold(mut self) -> Self {
        self.style = self.style.bold();
        self
    }
    /// Dim.
    pub fn dim(mut self) -> Self {
        self.style = self.style.dim();
        self
    }
    /// Italic.
    pub fn italic(mut self) -> Self {
        self.style = self.style.italic();
        self
    }
    /// Underline.
    pub fn underline(mut self) -> Self {
        self.style = self.style.underline();
        self
    }
    /// Slow blink.
    pub fn blink(mut self) -> Self {
        self.style = self.style.blink();
        self
    }
    /// Rapid blink.
    pub fn blink_fast(mut self) -> Self {
        self.style = self.style.blink_fast();
        self
    }
    /// Reverse video.
    pub fn reverse(mut self) -> Self {
        self.style = self.style.reverse();
        self
    }
    /// Hidden.
    pub fn hidden(mut self) -> Self {
        self.style = self.style.hidden();
        self
    }
    /// Strikethrough.
    pub fn strikethrough(mut self) -> Self {
        self.style = self.style.strikethrough();
        self
    }
}
