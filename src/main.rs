pub mod result;
pub mod utils;
pub mod word;

use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Parser)]
#[clap(name = "katype", author = "kadircy")]
pub struct Cli {
    /// The amount of words given from `generate_words` for typing test. Max value is 65535.
    #[clap(long, short = 'a', default_value_t = 15)]
    amount: u16,
}

fn main() {
    let args = Cli::parse();
    let stdin = stdin();
    let mut user_input = String::new();
    let words: word::Words = word::generate_words(word::Lang::English, args.amount);
    let (w_padding, h_padding) = utils::calculate_paddings(words.len());
    utils::clear_terminal();
    println!("{}", utils::colorize("Be ready", utils::Color::Red));
    std::thread::sleep(Duration::new(3, 0));
    utils::clear_terminal();
    print!("{}", "\n".repeat(h_padding));
    print!("{}", " ".repeat(w_padding));
    for word in &words {
        print!("{} ", *word);
    }
    println!("");
    print!("{}", " ".repeat(w_padding));
    let timer = Instant::now();
    stdout().flush().unwrap();
    stdin.read_line(&mut user_input).unwrap();
    let elapsed = timer.elapsed();
    let user_words = user_input.split_whitespace().collect::<Vec<&str>>();
    utils::clear_terminal();
    let mut word_statistic: String = String::new();
    for (index, word) in words.iter().enumerate() {
        let style: String =
            word::stylize_word(word, user_words.get(index).unwrap_or(&"").to_string());
        word_statistic.push_str(&style);
        word_statistic.push_str(" ");
    }
    let (wpm, acc, consistency): result::Results =
        result::calculate_result(&words, &user_words, elapsed.as_secs().try_into().unwrap());
    println!("{}", word_statistic);
    println!(
        "wpm: {}",
        utils::colorize(&wpm.to_string(), utils::Color::Green)
    );
    println!(
        "acc: {}",
        utils::colorize(&acc.to_string(), utils::Color::Green)
    );
    println!(
        "total {} chars in {} words",
        utils::colorize(
            &words
                .join("")
                .chars()
                .collect::<Vec<char>>()
                .len()
                .to_string(),
            utils::Color::Green
        ),
        utils::colorize(&words.len().to_string(), utils::Color::Green)
    );
    println!(
        "consistency: {}%",
        utils::colorize(&consistency.to_string(), utils::Color::Green)
    );
    println!(
        "type: {}s {}",
        utils::colorize(&elapsed.as_secs().to_string(), utils::Color::Green),
        utils::colorize("english", utils::Color::Green),
    );
}
