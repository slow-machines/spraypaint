//! Integration tests: verify exact ANSI byte sequences produced by spraypaint.
//!
//! All tests force TrueColor level so output is deterministic regardless of
//! the environment running the tests.

use spraypaint::detect::{set_color_level, ColorLevel};
use spraypaint::{styled, Color, Colorize, Style};

fn tc() {
    set_color_level(ColorLevel::TrueColor);
}
fn nc() {
    set_color_level(ColorLevel::None);
}

// ── Color rendering ───────────────────────────────────────────────────────────

#[test]
fn basic_red_foreground() {
    tc();
    let s = "hello".red().to_string();
    // ESC[31m = red foreground; ESC[0m = reset
    assert_eq!(s, "\x1b[31mhello\x1b[0m");
}

#[test]
fn basic_green_background() {
    tc();
    let s = "hello".on_green().to_string();
    assert_eq!(s, "\x1b[42mhello\x1b[0m");
}

#[test]
fn truecolor_rgb() {
    tc();
    let s = "test".rgb(255, 87, 51).to_string();
    assert_eq!(s, "\x1b[38;2;255;87;51mtest\x1b[0m");
}

#[test]
fn truecolor_hex() {
    tc();
    let s = "test".hex("#ff5733").to_string();
    assert_eq!(s, "\x1b[38;2;255;87;51mtest\x1b[0m");
}

#[test]
fn background_rgb() {
    tc();
    let s = "test".on_rgb(10, 20, 30).to_string();
    assert_eq!(s, "\x1b[48;2;10;20;30mtest\x1b[0m");
}

// ── Attribute rendering ───────────────────────────────────────────────────────

#[test]
fn bold_attribute() {
    tc();
    let s = "bold".bold().to_string();
    assert_eq!(s, "\x1b[1mbold\x1b[0m");
}

#[test]
fn italic_attribute() {
    tc();
    let s = "italic".italic().to_string();
    assert_eq!(s, "\x1b[3mitalic\x1b[0m");
}

#[test]
fn underline_attribute() {
    tc();
    let s = "under".underline().to_string();
    assert_eq!(s, "\x1b[4munder\x1b[0m");
}

#[test]
fn strikethrough_attribute() {
    tc();
    let s = "strike".strikethrough().to_string();
    assert_eq!(s, "\x1b[9mstrike\x1b[0m");
}

// ── Combined styles ───────────────────────────────────────────────────────────

#[test]
fn bold_red() {
    tc();
    let s = "error".red().bold().to_string();
    // Attributes come before color codes: bold=1, red=31
    assert_eq!(s, "\x1b[1;31merror\x1b[0m");
}

#[test]
fn red_on_blue_italic() {
    tc();
    let s = "warn".red().on_blue().italic().to_string();
    // italic=3, red=31, blue bg=44
    assert_eq!(s, "\x1b[3;31;44mwarn\x1b[0m");
}

#[test]
fn style_builder() {
    tc();
    let sty = Style::new().green().bold().underline();
    let s = sty.apply("go").to_string();
    assert_eq!(s, "\x1b[1;4;32mgo\x1b[0m");
}

// ── Color level downgrade ─────────────────────────────────────────────────────

#[test]
fn no_color_strips_all_ansi() {
    nc();
    let s = "hello".red().bold().italic().to_string();
    assert_eq!(s, "hello");
    tc();
}

#[test]
fn xterm256_level_uses_8bit_codes() {
    set_color_level(ColorLevel::Xterm256);
    // Xterm 256 level: RGB should be downgraded to xterm256 index
    let s = "hi".rgb(255, 0, 0).to_string();
    // pure red → xterm index 196
    assert_eq!(s, "\x1b[38;5;196mhi\x1b[0m");
    tc();
}

#[test]
fn basic16_level_downgrades_rgb() {
    set_color_level(ColorLevel::Basic16);
    // RGB red → ANSI red (31)
    let s = "hi".rgb(200, 10, 10).to_string();
    assert_eq!(s, "\x1b[31mhi\x1b[0m");
    tc();
}

// ── paint! macro ──────────────────────────────────────────────────────────────

#[test]
fn styled_macro_literal() {
    tc();
    let s = styled!("{red.bold Error:} all good");
    assert!(s.contains("\x1b[1;31mError:\x1b[0m"));
    assert!(s.contains(" all good"));
}

#[test]
fn styled_macro_nested() {
    tc();
    let s = styled!("{blue Welcome to {bold.underline spraypaint}}");
    assert!(s.contains("spraypaint"));
    assert!(s.contains("\x1b["));
}

#[test]
fn styled_macro_with_expression() {
    tc();
    let name = "world";
    let s = styled!("Hello {green {name}}!");
    assert!(s.contains("world"));
    assert!(s.contains("\x1b[32m"));
}

#[test]
fn styled_macro_plain_text() {
    tc();
    let s = styled!("no styling here");
    assert_eq!(s, "no styling here");
}

// ── Hex color parsing ─────────────────────────────────────────────────────────

#[test]
fn hex_3digit_shorthand() {
    tc();
    // #f00 = #ff0000 = rgb(255, 0, 0)
    let s = "x".hex("#f00").to_string();
    assert_eq!(s, "\x1b[38;2;255;0;0mx\x1b[0m");
}

// ── Gradient ──────────────────────────────────────────────────────────────────

#[test]
fn gradient_length_matches_input() {
    tc();
    let g = "hello".gradient(Color::RED, Color::CYAN);
    let rendered = g.to_string();
    // Each character should have its own ESC sequence, so there should be
    // at least 5 resets (one per character).
    assert_eq!(rendered.matches("\x1b[0m").count(), 5);
}

#[test]
fn gradient_no_color_strips_ansi() {
    nc();
    let g = "hello".gradient(Color::RED, Color::CYAN);
    let rendered = g.to_string();
    assert_eq!(rendered, "hello");
    tc();
}

// ── Colorize on non-str types ─────────────────────────────────────────────────

#[test]
fn colorize_on_integer() {
    tc();
    let s = 42_i32.red().to_string();
    assert_eq!(s, "\x1b[31m42\x1b[0m");
}

#[test]
fn colorize_on_string_type() {
    tc();
    let owned = String::from("owned");
    let s = owned.green().bold().to_string();
    assert_eq!(s, "\x1b[1;32mowned\x1b[0m");
}
