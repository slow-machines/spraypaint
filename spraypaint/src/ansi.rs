//! Low-level ANSI SGR (Select Graphic Rendition) escape code writing.
//!
//! All output goes through a single `write_ansi` function so that the rendering
//! path is easy to swap out (e.g., for a no-color level).

use std::fmt;

use crate::color::{AnsiColor, Color};
use crate::detect::ColorLevel;
use crate::style::{Attrs, Style};

/// ANSI escape sequence prefix.
const ESC: &str = "\x1b[";
/// SGR reset code.
const RESET: &str = "\x1b[0m";

/// Write the opening ANSI codes for `style` into `f`, respecting the active color level.
pub(crate) fn write_open(f: &mut fmt::Formatter<'_>, style: &Style) -> fmt::Result {
    let level = crate::detect::color_level();

    if level == ColorLevel::None {
        return Ok(());
    }

    // Collect all SGR parameters into a small buffer to emit one escape sequence.
    let mut params: arrayvec::ArrayVec<u8, 12> = arrayvec::ArrayVec::new();

    // Attributes
    if style.attrs.contains(Attrs::BOLD) {
        params.push(1);
    }
    if style.attrs.contains(Attrs::DIM) {
        params.push(2);
    }
    if style.attrs.contains(Attrs::ITALIC) {
        params.push(3);
    }
    if style.attrs.contains(Attrs::UNDERLINE) {
        params.push(4);
    }
    if style.attrs.contains(Attrs::BLINK) {
        params.push(5);
    }
    if style.attrs.contains(Attrs::BLINK_FAST) {
        params.push(6);
    }
    if style.attrs.contains(Attrs::REVERSE) {
        params.push(7);
    }
    if style.attrs.contains(Attrs::HIDDEN) {
        params.push(8);
    }
    if style.attrs.contains(Attrs::STRIKETHROUGH) {
        params.push(9);
    }

    // Foreground color
    if let Some(fg) = style.fg {
        write_color_params(f, fg, false, level, &mut params)?;
    }

    // Background color
    if let Some(bg) = style.bg {
        write_color_params(f, bg, true, level, &mut params)?;
    }

    // Emit nothing if there's nothing to apply
    if params.is_empty() {
        return Ok(());
    }

    // Write a single ESC[ ... m sequence
    write!(f, "{ESC}")?;
    for (i, &p) in params.iter().enumerate() {
        if i > 0 {
            write!(f, ";")?;
        }
        write!(f, "{p}")?;
    }
    write!(f, "m")
}

/// Write the ANSI reset sequence.
pub(crate) fn write_close(f: &mut fmt::Formatter<'_>, style: &Style) -> fmt::Result {
    let level = crate::detect::color_level();
    if level == ColorLevel::None {
        return Ok(());
    }
    // Only write reset if the style actually does something.
    let has_style = style.fg.is_some() || style.bg.is_some() || !style.attrs.is_empty();
    if has_style {
        write!(f, "{RESET}")?;
    }
    Ok(())
}

/// Downgrade a color to the supported level and push its SGR parameters.
/// `is_bg` true means we're targeting the background.
fn write_color_params(
    f: &mut fmt::Formatter<'_>,
    color: Color,
    is_bg: bool,
    level: ColorLevel,
    params: &mut arrayvec::ArrayVec<u8, 12>,
) -> fmt::Result {
    match color {
        Color::Ansi(c) => {
            let code = if is_bg { c.bg_code() } else { c.fg_code() };
            params.push(code);
        }
        Color::Xterm(idx) => {
            if level >= ColorLevel::Xterm256 {
                if is_bg {
                    // Flush params so far, write 256-color sequence inline
                    flush_params(f, params)?;
                    write!(f, "{ESC}48;5;{idx}m")?;
                } else {
                    flush_params(f, params)?;
                    write!(f, "{ESC}38;5;{idx}m")?;
                }
            } else {
                // Downgrade to nearest ANSI-16 color
                let ansi = xterm_to_ansi16(idx);
                let code = if is_bg {
                    ansi.bg_code()
                } else {
                    ansi.fg_code()
                };
                params.push(code);
            }
        }
        Color::Rgb(r, g, b) => {
            if level >= ColorLevel::TrueColor {
                flush_params(f, params)?;
                if is_bg {
                    write!(f, "{ESC}48;2;{r};{g};{b}m")?;
                } else {
                    write!(f, "{ESC}38;2;{r};{g};{b}m")?;
                }
            } else if level >= ColorLevel::Xterm256 {
                let idx = rgb_to_xterm256(r, g, b);
                flush_params(f, params)?;
                if is_bg {
                    write!(f, "{ESC}48;5;{idx}m")?;
                } else {
                    write!(f, "{ESC}38;5;{idx}m")?;
                }
            } else {
                let ansi = rgb_to_ansi16(r, g, b);
                let code = if is_bg {
                    ansi.bg_code()
                } else {
                    ansi.fg_code()
                };
                params.push(code);
            }
        }
    }
    Ok(())
}

/// Emit any buffered single-byte params as a partial SGR sequence, then clear the buffer.
fn flush_params(
    f: &mut fmt::Formatter<'_>,
    params: &mut arrayvec::ArrayVec<u8, 12>,
) -> fmt::Result {
    if params.is_empty() {
        return Ok(());
    }
    write!(f, "{ESC}")?;
    for (i, &p) in params.iter().enumerate() {
        if i > 0 {
            write!(f, ";")?;
        }
        write!(f, "{p}")?;
    }
    write!(f, "m")?;
    params.clear();
    Ok(())
}

/// Best-effort mapping from an Xterm-256 index to the nearest ANSI-16 name.
fn xterm_to_ansi16(idx: u8) -> AnsiColor {
    // The first 16 indices are exactly the ANSI colors.
    match idx {
        0 => AnsiColor::Black,
        1 => AnsiColor::Red,
        2 => AnsiColor::Green,
        3 => AnsiColor::Yellow,
        4 => AnsiColor::Blue,
        5 => AnsiColor::Magenta,
        6 => AnsiColor::Cyan,
        7 => AnsiColor::White,
        8 => AnsiColor::BrightBlack,
        9 => AnsiColor::BrightRed,
        10 => AnsiColor::BrightGreen,
        11 => AnsiColor::BrightYellow,
        12 => AnsiColor::BrightBlue,
        13 => AnsiColor::BrightMagenta,
        14 => AnsiColor::BrightCyan,
        15 => AnsiColor::BrightWhite,
        // For the 216-color cube and grayscale ramp, map to nearest brightness.
        16..=231 => {
            let v = idx - 16;
            let brightness = (v / 36) * 4 + ((v % 36) / 6) * 2 + (v % 6);
            if brightness > 10 {
                AnsiColor::White
            } else {
                AnsiColor::Black
            }
        }
        232..=255 => {
            if idx > 243 {
                AnsiColor::White
            } else {
                AnsiColor::Black
            }
        }
    }
}

/// Map an RGB color to the nearest Xterm 256-color cube index.
pub(crate) fn rgb_to_xterm256(r: u8, g: u8, b: u8) -> u8 {
    // Grayscale ramp check (16 steps, indices 232-255)
    let avg = (r as u16 + g as u16 + b as u16) / 3;
    if (r as i16 - avg as i16).abs() < 8
        && (g as i16 - avg as i16).abs() < 8
        && (b as i16 - avg as i16).abs() < 8
        && avg < 248
        && avg > 7
    {
        let gs = ((avg - 8) as f32 / 247.0 * 24.0).round() as u8;
        return 232 + gs.min(23);
    }
    // 6x6x6 color cube (indices 16-231)
    let ri = ((r as f32 / 255.0) * 5.0).round() as u8;
    let gi = ((g as f32 / 255.0) * 5.0).round() as u8;
    let bi = ((b as f32 / 255.0) * 5.0).round() as u8;
    16 + 36 * ri + 6 * gi + bi
}

/// Map an RGB color to the nearest ANSI-16 name using simple luminance bucketing.
fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> AnsiColor {
    let bright = (r as u16 + g as u16 + b as u16) > 382;
    let dominant_r = r > g && r > b;
    let dominant_g = g > r && g > b;
    let dominant_b = b > r && b > g;
    let is_gray = (r as i16 - g as i16).abs() < 32 && (g as i16 - b as i16).abs() < 32;

    if is_gray {
        if bright {
            AnsiColor::White
        } else {
            AnsiColor::BrightBlack
        }
    } else if dominant_r && g > 100 {
        if bright {
            AnsiColor::BrightYellow
        } else {
            AnsiColor::Yellow
        }
    } else if dominant_r {
        if bright {
            AnsiColor::BrightRed
        } else {
            AnsiColor::Red
        }
    } else if dominant_g && b > 100 {
        if bright {
            AnsiColor::BrightCyan
        } else {
            AnsiColor::Cyan
        }
    } else if dominant_g {
        if bright {
            AnsiColor::BrightGreen
        } else {
            AnsiColor::Green
        }
    } else if dominant_b && r > 100 {
        if bright {
            AnsiColor::BrightMagenta
        } else {
            AnsiColor::Magenta
        }
    } else if bright {
        AnsiColor::BrightBlue
    } else {
        AnsiColor::Blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xterm256_from_rgb() {
        // Pure red should map to index 196 in the 6x6x6 cube
        let idx = rgb_to_xterm256(255, 0, 0);
        assert_eq!(idx, 196);
    }

    #[test]
    fn grayscale_ramp() {
        // Mid-gray should land in the grayscale ramp (232-255)
        let idx = rgb_to_xterm256(128, 128, 128);
        assert!(idx >= 232, "expected grayscale, got {idx}");
    }
}
