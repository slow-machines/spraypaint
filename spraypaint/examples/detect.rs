//! cargo run --example detect
//!
//! Try: NO_COLOR=1, FORCE_COLOR=3, COLORTERM=truecolor

use spraypaint::{color_level, set_color_level, ColorLevel, Colorize};

fn main() {
    let level = color_level();
    println!("Detected color level: {:?}", level);

    println!();
    println!("--- Rendering at detected level ---");
    render_samples();

    println!();
    println!("--- Forcing TrueColor ---");
    set_color_level(ColorLevel::TrueColor);
    render_samples();

    println!();
    println!("--- Forcing Xterm256 ---");
    set_color_level(ColorLevel::Xterm256);
    render_samples();

    println!();
    println!("--- Forcing Basic16 ---");
    set_color_level(ColorLevel::Basic16);
    render_samples();

    println!();
    println!("--- Forcing No Color ---");
    set_color_level(ColorLevel::None);
    render_samples();
}

fn render_samples() {
    "Truecolor RGB (255, 87, 51)".rgb(255, 87, 51).paint();
    "Hex color #6c5ce7".hex("#6c5ce7").paint();
    "Basic red".red().bold().paint();
}
