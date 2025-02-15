// Modules related to the core functionality
pub mod code;
pub mod result;
pub mod utils;
pub mod word;

use clap::Parser;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_READY_TEXT: &str = "Be ready";

#[derive(serde::Serialize)]
struct ResultJson {
    wpm: f32,
    acc: f32,
    consistency: f32,
}

// Define the command-line arguments structure.
#[derive(Debug, Parser)]
#[clap(
    name = "katype",
    about = "A fast typing test from terminal writted in Rust 🦀",
    author = "kadircy",
    version = "0.2.1"
)]
struct Cli {
    /// The number of words generated for the typing test. Max value is 65535.
    #[clap(long, short = 'a', default_value_t = 15)]
    amount: u16,

    /// The language of words for the typing test. Available langs are: EN, ES.
    #[clap(long, short = 'l', default_value_t = String::from("en"))]
    lang: String,

    /// Use a code instead of random words. Overrides other options and uses parameters from the code.
    #[clap(long, short = 'c')]
    code: Option<String>,

    /// The warning message before text starts.
    #[clap(long, short = 'r')]
    ready_text: Option<String>,

    /// Set a timeout for test. The test will be ended automatically when reached the timeout.
    #[clap(long, short = 't')]
    timeout: Option<u64>,

    /// Generate your code for custom typing test. Words are seperated by comma (,).
    #[clap(long, short = 'g')]
    generate: Option<String>,

    /// Print test results in JSON format. Useful when embedding results to your program.
    #[clap(long, short = 'j', default_value_t = false)]
    json: bool,
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

    // Check if there is some generate argument
    if args.generate.is_some() {
        // Generate Base64 code for words
        let code = code::generate_code_from_str(&args.generate.unwrap());
        println!(
            "There is your code: {}",
            utils::colorize(&code, utils::Color::Green)
        );
        println!(
            "You can run test with this code via: {}",
            utils::colorize(&format!("katype --code {}", &code), utils::Color::Green)
        );
        // Exit after code generated before terminal cleaned
        std::process::exit(0);
    }

    // Resolve words either from code or by generating random words
    let words = if let Some(code) = &args.code {
        code::resolve_code(code)
    } else {
        word::generate_words(&lang, args.amount)
    };

    let ready_text = args.ready_text.unwrap_or(String::from(DEFAULT_READY_TEXT));

    let (_, h_padding_ready_text) = utils::calculate_paddings(ready_text.len());

    // Centerize the words and user input in terminal
    let (w_padding, h_padding) = utils::calculate_paddings(words.len());
    utils::clear_terminal();

    // Show a warning message before the timer starts
    print!("{}", "\n".repeat(h_padding_ready_text)); // Padding for height
    print!("{:width$}", "", width = w_padding); // Padding for ready text
    println!("{}", utils::colorize(&ready_text, utils::Color::Red));
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
    // Start the timeout thread if timout argument given
    if args.timeout.is_some() {
        thread::spawn(move || loop {
            if timer.elapsed().as_secs() >= Duration::new(args.timeout.unwrap(), 0).as_secs() {
                println!(
                    "{}",
                    utils::colorize("The timeout ended.", utils::Color::Red)
                );
                std::process::exit(0);
            }
        });
    }

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap(); // Read user input

    let elapsed = timer.elapsed(); // Get the elapsed time
    let user_words: Vec<&str> = user_input.split_whitespace().collect(); // Split user input into words

    utils::clear_terminal();

    // Calculate the results for the typing test
    let (wpm, acc, consistency) =
        result::calculate_result(&words, &user_words, elapsed.as_secs() as u16);

    if args.json {
        // Don't need to calculate word_statistics
        // Because we will not use it (for now)
        let json = serde_json::to_string(&ResultJson {
            wpm,
            acc,
            consistency,
        });
        println!("{}", json.expect("Unable to convert result to json"));
        // Exit from program because we don't want to print more information to stdout
        std::process::exit(0);
    }

    // Build the word statistics string for colored output
    let word_statistic: String = words
        .iter()
        .zip(user_words.iter()) // Zip together words and user input for comparison
        .map(|(word, user_word)| word::stylize_word(word, user_word.to_string()) + " ")
        .collect();

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
        utils::colorize(&consistency.to_string(), utils::Color::Green),
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
