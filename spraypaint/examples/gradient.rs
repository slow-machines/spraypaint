//! cargo run --example gradient

use spraypaint::{Color, Colorize};

fn main() {
    "Red to Cyan".gradient(Color::RED, Color::CYAN).paint();
    "Blue to Green".gradient(Color::BLUE, Color::GREEN).paint();
    "Magenta to Yellow"
        .gradient(Color::MAGENTA, Color::YELLOW)
        .paint();

    println!();

    let rainbow_stops = vec![
        Color::rgb(255, 0, 0),
        Color::rgb(255, 165, 0),
        Color::rgb(255, 255, 0),
        Color::rgb(0, 255, 0),
        Color::rgb(0, 0, 255),
        Color::rgb(75, 0, 130),
        Color::rgb(148, 0, 211),
    ];
    "The quick brown fox jumps over the lazy dog"
        .gradient_multi(rainbow_stops)
        .paint();

    println!();

    let hex_from = Color::from_hex("#6c5ce7").unwrap();
    let hex_to = Color::from_hex("#fd79a8").unwrap();
    "Purple to Pink gradient".gradient(hex_from, hex_to).paint();

    println!();

    use spraypaint::Style;
    "Bold gradient"
        .gradient(Color::CYAN, Color::MAGENTA)
        .with_style(Style::new().bold())
        .paint();

    "Hi".gradient(Color::RED, Color::BLUE).paint();
    "X".gradient(Color::GREEN, Color::YELLOW).paint();
}
