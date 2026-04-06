//! Gradient text: interpolate colors smoothly across a string's characters.

use std::fmt;

use crate::ansi;
use crate::color::Color;
use crate::detect::ColorLevel;
use crate::style::Style;

/// A string with a gradient applied across its characters.
///
/// Created via `.gradient()` or `.gradient_multi()` on the
/// [`Colorize`](crate::Colorize) trait.
///
/// The gradient is rendered per **visible character** (Unicode scalar value),
/// so multibyte characters are handled correctly. ANSI codes are emitted
/// character-by-character so the color smoothly transitions across the text.
pub struct Gradient {
    pub(crate) text: String,
    pub(crate) stops: Vec<Color>,
    pub(crate) base_style: Style,
}

impl Gradient {
    /// Create a two-stop gradient from `from` to `to`.
    pub fn new(text: impl Into<String>, from: Color, to: Color) -> Self {
        Self {
            text: text.into(),
            stops: vec![from, to],
            base_style: Style::new(),
        }
    }

    /// Create a multi-stop gradient.
    ///
    /// # Panics
    /// Panics if `stops` contains fewer than two elements.
    pub fn multi_stop(text: impl Into<String>, stops: Vec<Color>) -> Self {
        assert!(
            stops.len() >= 2,
            "gradient requires at least two color stops"
        );
        Self {
            text: text.into(),
            stops,
            base_style: Style::new(),
        }
    }

    /// Apply additional text attributes (bold, italic, etc.) on top of the gradient.
    pub fn with_style(mut self, style: Style) -> Self {
        self.base_style = self.base_style.merge(style);
        self
    }

    /// Print the gradient text to stdout with a trailing newline.
    pub fn paint(&self) {
        println!("{self}");
    }

    /// Print the gradient text to stdout **without** a trailing newline.
    pub fn paint_inline(&self) {
        print!("{self}");
    }

    /// Print the gradient text to stderr with a trailing newline.
    pub fn paint_err(&self) {
        eprintln!("{self}");
    }

    // ── Attribute chaining ────────────────────────────────────────────────────

    /// Apply bold to the gradient text.
    pub fn bold(self) -> Self {
        self.with_style(Style::new().bold())
    }
    /// Apply dim to the gradient text.
    pub fn dim(self) -> Self {
        self.with_style(Style::new().dim())
    }
    /// Apply italic to the gradient text.
    pub fn italic(self) -> Self {
        self.with_style(Style::new().italic())
    }
    /// Apply underline to the gradient text.
    pub fn underline(self) -> Self {
        self.with_style(Style::new().underline())
    }
    /// Apply slow blink to the gradient text.
    pub fn blink(self) -> Self {
        self.with_style(Style::new().blink())
    }
    /// Apply rapid blink to the gradient text.
    pub fn blink_fast(self) -> Self {
        self.with_style(Style::new().blink_fast())
    }
    /// Apply reverse video to the gradient text.
    pub fn reverse(self) -> Self {
        self.with_style(Style::new().reverse())
    }
    /// Apply strikethrough to the gradient text.
    pub fn strikethrough(self) -> Self {
        self.with_style(Style::new().strikethrough())
    }
}

impl fmt::Display for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = crate::detect::color_level();

        // When color is disabled, just write the text.
        if level == ColorLevel::None {
            return write!(f, "{}", self.text);
        }

        let chars: Vec<char> = self.text.chars().collect();
        let n = chars.len();

        if n == 0 {
            return Ok(());
        }

        for (i, &ch) in chars.iter().enumerate() {
            let t = if n == 1 {
                0.0_f32
            } else {
                i as f32 / (n - 1) as f32
            };
            let color = interpolate(&self.stops, t);
            let style = self.base_style.fg(color);
            ansi::write_open(f, &style)?;
            write!(f, "{ch}")?;
            ansi::write_close(f, &style)?;
        }
        Ok(())
    }
}

/// Interpolate among a list of color stops at position `t ∈ [0, 1]`.
fn interpolate(stops: &[Color], t: f32) -> Color {
    let segments = stops.len() - 1;
    let scaled = t * segments as f32;
    let segment = (scaled.floor() as usize).min(segments - 1);
    let local_t = scaled - segment as f32;

    let a = to_rgb(stops[segment]);
    let b = to_rgb(stops[segment + 1]);

    let r = lerp(a.0, b.0, local_t);
    let g = lerp(a.1, b.1, local_t);
    let bl = lerp(a.2, b.2, local_t);

    Color::Rgb(r, g, bl)
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

/// Convert any [`Color`] to an RGB triple for interpolation purposes.
fn to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Xterm(idx) => xterm256_to_rgb(idx),
        Color::Ansi(c) => ansi_to_rgb(c),
    }
}

/// Approximate RGB values for the 256-color Xterm palette.
fn xterm256_to_rgb(idx: u8) -> (u8, u8, u8) {
    match idx {
        0 => (0, 0, 0),
        1 => (128, 0, 0),
        2 => (0, 128, 0),
        3 => (128, 128, 0),
        4 => (0, 0, 128),
        5 => (128, 0, 128),
        6 => (0, 128, 128),
        7 => (192, 192, 192),
        8 => (128, 128, 128),
        9 => (255, 0, 0),
        10 => (0, 255, 0),
        11 => (255, 255, 0),
        12 => (0, 0, 255),
        13 => (255, 0, 255),
        14 => (0, 255, 255),
        15 => (255, 255, 255),
        16..=231 => {
            let v = idx - 16;
            let ri = v / 36;
            let gi = (v % 36) / 6;
            let bi = v % 6;
            let scale = |i: u8| if i == 0 { 0 } else { 55 + i * 40 };
            (scale(ri), scale(gi), scale(bi))
        }
        232..=255 => {
            let level = 8 + (idx - 232) * 10;
            (level, level, level)
        }
    }
}

/// Approximate RGB values for the 16 named ANSI colors.
fn ansi_to_rgb(c: crate::color::AnsiColor) -> (u8, u8, u8) {
    use crate::color::AnsiColor::{
        Black, Blue, BrightBlack, BrightBlue, BrightCyan, BrightGreen, BrightMagenta, BrightRed,
        BrightWhite, BrightYellow, Cyan, Green, Magenta, Red, White, Yellow,
    };
    match c {
        Black => (0, 0, 0),
        Red => (170, 0, 0),
        Green => (0, 170, 0),
        Yellow => (170, 170, 0),
        Blue => (0, 0, 170),
        Magenta => (170, 0, 170),
        Cyan => (0, 170, 170),
        White => (170, 170, 170),
        BrightBlack => (85, 85, 85),
        BrightRed => (255, 85, 85),
        BrightGreen => (85, 255, 85),
        BrightYellow => (255, 255, 85),
        BrightBlue => (85, 85, 255),
        BrightMagenta => (255, 85, 255),
        BrightCyan => (85, 255, 255),
        BrightWhite => (255, 255, 255),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_midpoint() {
        let stops = vec![Color::Rgb(0, 0, 0), Color::Rgb(100, 200, 50)];
        let mid = interpolate(&stops, 0.5);
        assert_eq!(mid, Color::Rgb(50, 100, 25));
    }

    #[test]
    fn interpolate_start_end() {
        let stops = vec![Color::Rgb(255, 0, 0), Color::Rgb(0, 0, 255)];
        assert_eq!(interpolate(&stops, 0.0), Color::Rgb(255, 0, 0));
        assert_eq!(interpolate(&stops, 1.0), Color::Rgb(0, 0, 255));
    }

    #[test]
    fn multi_stop_three_colors() {
        let stops = vec![
            Color::Rgb(255, 0, 0),
            Color::Rgb(0, 255, 0),
            Color::Rgb(0, 0, 255),
        ];
        let c = interpolate(&stops, 0.25);
        assert_eq!(c, Color::Rgb(128, 128, 0));
    }
}
