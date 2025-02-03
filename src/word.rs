use crate::utils::{colorize, Color};
use random_word::gen;
use random_word::Lang;

pub type Word = &'static str;
pub type Words = Vec<Word>;

pub fn generate_words(amount: u16) -> Words {
    let mut words: Words = Vec::new();
    // Loop for amount and add random generated words to Vec.
    for _ in 0..amount {
        let word: Word = gen(Lang::En);
        words.push(word);
    }
    return words;
}

pub fn stylize_word(word: Word, user_type: String) -> String {
    let mut styled_word: String = String::new(); // Create a new string for styled text.
    let word_chars: Vec<char> = word.chars().collect();
    let user_type_chars: Vec<char> = user_type.chars().collect();
    // Loop for characters and index in `word_chars`.
    for (index, ch) in word_chars.iter().enumerate() {
        let user_ch = user_type_chars.get(index);
        // If there is no typed char from user.
        if user_ch.is_none() {
            styled_word.push_str(&colorize(&ch.to_string(), Color::Gray));
            continue;
        }

        // If the user typed char and word is same.
        if user_ch.unwrap() == ch {
            styled_word.push_str(&colorize(&ch.to_string(), Color::Green));
            continue;
        }

        // If the user typed char and word is different.
        if user_ch.unwrap() != ch {
            styled_word.push_str(&colorize(&ch.to_string(), Color::Red));
        }
    }
    return styled_word;
}
