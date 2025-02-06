pub mod result;
pub mod utils;
pub mod word;
use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Parser)]
#[clap(name = "katype", author = "kadircy", version = "0.1.1")]
pub struct Cli {
    /// The amount of words given from `generate_words` for typing test. Max value is 65535.
    #[clap(long, short = 'a', default_value_t = 15)]
    amount: u16,
    /// The language of words for typing test. Avaliable langs are: EN, ES
    #[clap(long, short = 'l', default_value_t = String::from("en"))]
    lang: String,
}

fn main() {
    let args = Cli::parse();
    let stdin = stdin();
    let mut user_input = String::new();
    let lang: word::Lang = match &args.lang as &str {
        "en" => word::Lang::English,
        "es" => word::Lang::Spanish,
        _ => {
            utils::clear_terminal();
            println!("Unimplemented langauge. Exiting with code 1.");
            std::process::exit(1);
        }
    };
    let words: word::Words = word::generate_words(&lang, args.amount);
    // Centerize words and user input in terminal.
    let (w_padding, h_padding) = utils::calculate_paddings(words.len());
    utils::clear_terminal();
    // Show warning message because the timer start instantly.
    println!("{}", utils::colorize("Be ready", utils::Color::Red));
    std::thread::sleep(Duration::new(3, 0));
    utils::clear_terminal();
    print!("{}", "\n".repeat(h_padding));
    print!("{}", " ".repeat(w_padding));
    // Print words with an space.
    for word in &words {
        print!("{} ", *word);
    }
    println!("");
    // Add width padding for user input to centerize.
    print!("{}", " ".repeat(w_padding));
    // Start the timer on last change to get better results.
    let timer = Instant::now();
    stdout().flush().unwrap();
    stdin.read_line(&mut user_input).unwrap();
    let elapsed = timer.elapsed();
    // Split words with whitespaces.
    let user_words = user_input.split_whitespace().collect::<Vec<&str>>();
    utils::clear_terminal();
    // Create an String for word statistics to show.
    let mut word_statistic: String = String::new();
    for (index, word) in words.iter().enumerate() {
        // Colorize statistic word.
        let style: String =
            word::stylize_word(word, user_words.get(index).unwrap_or(&"").to_string());
        word_statistic.push_str(&style);
        word_statistic.push_str(" ");
    }
    // Get results for typing test.
    let (wpm, acc, consistency): result::Results =
        result::calculate_result(&words, &user_words, elapsed.as_secs().try_into().unwrap());
    // Print results to the terminal for user to see.
    println!("{}", word_statistic);
    println!(
        "wpm: {}",
        utils::colorize(&wpm.to_string(), utils::Color::Green)
    );
    println!(
        "acc: {}%",
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
        utils::colorize(&lang.to_string(), utils::Color::Green),
    );
}
