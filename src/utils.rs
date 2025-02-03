use std::cmp::max;
use std::env;

pub enum Color {
    Gray,
    Red,
    Green,
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn colorize(text: &str, color: Color) -> String {
    match color {
        Color::Gray => format!("\x1b[38;5;245m{}\x1b[0m", text),
        Color::Red => format!("\x1b[31m{}\x1b[0m", text),
        Color::Green => format!("\x1b[32m{}\x1b[0m", text),
    }
}

pub fn calculate_paddings(text_len: usize) -> (usize, usize) {
    // Get terminal width and height from environment variables
    let terminal_width = env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(80);
    let terminal_height = env::var("LINES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(24);
    let width_padding = max((terminal_width - text_len) / 8, 7);
    let height_padding = max((terminal_height - 3) / 2, 7);

    return (width_padding, height_padding);
}
