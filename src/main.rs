// Modules related to the core functionality
pub mod code;
pub mod result;
pub mod utils;
pub mod word;

use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};

// Define the command-line arguments structure.
#[derive(Debug, Parser)]
#[clap(name = "katype", author = "kadircy", version = "0.1.1")]
pub struct Cli {
    /// The number of words generated for the typing test. Max value is 65535.
    #[clap(long, short = 'a', default_value_t = 15)]
    amount: u16,

    /// The language of words for the typing test. Available langs are: EN, ES.
    #[clap(long, short = 'l', default_value_t = String::from("en"))]
    lang: String,

    /// Use a code instead of random words. Overrides other options and uses parameters from the code.
    #[clap(long, short = 'c')]
    code: Option<String>,
}

fn main() {
    let args = Cli::parse(); // Parse command-line arguments

    // Setup the language based on the user input
    let lang: word::Lang = match args.lang.as_str() {
        "en" => word::Lang::English,
        "es" => word::Lang::Spanish,
        _ => {
            utils::clear_terminal();
            println!("Unimplemented language. Exiting with code 1.");
            std::process::exit(1);
        }
    };

    // Resolve words either from code or by generating random words
    let words = if let Some(code) = &args.code {
        code::resolve_code(code)
    } else {
        word::generate_words(&lang, args.amount)
    };

    // Centerize the words and user input in terminal
    let (w_padding, h_padding) = utils::calculate_paddings(words.len());
    utils::clear_terminal();

    // Show a warning message before the timer starts
    println!("{}", utils::colorize("Be ready", utils::Color::Red));
    std::thread::sleep(Duration::new(3, 0));
    utils::clear_terminal();

    // Print padding and words to the terminal
    print!("{}", "\n".repeat(h_padding)); // Padding for height
    print!("{:width$}", "", width = w_padding); // Padding before words
    for word in &words {
        print!("{} ", word);
    }
    println!();

    print!("{:width$}", "", width = w_padding); // Padding before user input
    stdout().flush().unwrap(); // Flush to make sure output is visible before input

    // Start the timer
    let timer = Instant::now();
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap(); // Read user input

    let elapsed = timer.elapsed(); // Get the elapsed time
    let user_words: Vec<&str> = user_input.split_whitespace().collect(); // Split user input into words

    utils::clear_terminal();

    // Build the word statistics string for colored output
    let word_statistic: String = words
        .iter()
        .zip(user_words.iter()) // Zip together words and user input for comparison
        .map(|(word, user_word)| word::stylize_word(word, user_word.to_string()) + " ")
        .collect();

    // Calculate the results for the typing test
    let (wpm, acc, consistency) =
        result::calculate_result(&words, &user_words, elapsed.as_secs() as u16);

    // Generate base64 code for the words
    let base64_code = code::generate_code(&words);

    // Print the results in a clean, formatted way
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
        utils::colorize(&words.join("").len().to_string(), utils::Color::Green),
        utils::colorize(&words.len().to_string(), utils::Color::Green)
    );
    println!(
        "consistency: {}%",
        utils::colorize(&consistency.to_string(), utils::Color::Green)
    );
    println!(
        "time: {}s {}",
        utils::colorize(&elapsed.as_secs().to_string(), utils::Color::Green),
        utils::colorize(&lang.to_string(), utils::Color::Green),
    );
    println!(
        "code: {}",
        utils::colorize(&base64_code, utils::Color::Green)
    );
}
