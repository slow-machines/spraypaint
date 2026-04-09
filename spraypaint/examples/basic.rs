//! cargo run --example basic

use spraypaint::Colorize;

fn main() {
    "black".black().on_white().paint();
    "red".red().paint();
    "green".green().paint();
    "yellow".yellow().paint();
    "blue".blue().paint();
    "magenta".magenta().paint();
    "cyan".cyan().paint();
    "white".white().on_black().paint();

    println!();

    "bright red".bright_red().paint();
    "bright green".bright_green().paint();
    "bright cyan".bright_cyan().paint();

    println!();

    "bold".bold().paint();
    "italic".italic().paint();
    "underline".underline().paint();
    "strikethrough".strikethrough().paint();
    "dim".dim().paint();

    println!();

    "bold red error".red().bold().paint();
    "italic green note".green().italic().paint();
    "underline blue link".blue().underline().paint();

    println!();

    "warning".black().on_yellow().bold().paint();
    "danger".white().on_red().bold().paint();
    "success".black().on_bright_green().paint();

    println!();

    "custom rgb".rgb(255, 87, 51).paint();
    "hex color".hex("#6c5ce7").paint();
    "rgb background".black().on_rgb(253, 203, 110).paint();

    println!();

    let status = "OK".green().bold();
    let code = 200_u16.cyan();
    println!("HTTP {} {}", code, status);

    use spraypaint::Style;
    let error_style = Style::new().red().bold().underline();
    error_style.apply("CRITICAL ERROR").paint();
    error_style.apply("ANOTHER ERROR").paint();
}
