use crate::utils::{colorize, Color};
use rand::prelude::IndexedRandom;
use std::fmt;

pub type Word = &'static str;
pub type Words = Vec<Word>;

pub enum Lang {
    English,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Lang::English => "en",
        };
        return write!(f, "{}", s);
    }
}

/// Returns most used 120 keywords on English language.
/// This is a private function because functions don't use it expect `generate_words`.
/// NOTE: All other functions similar to this should be named in this format: `{lang}_words`.
fn en_words() -> Words {
    let words: Vec<&'static str> = vec![
        "a", "and", "away", "big", "blue", "can", "come", "down", "find", "for", "funny", "go",
        "help", "here", "I", "in", "is", "it", "jump", "little", "look", "make", "me", "my", "not",
        "one", "play", "red", "run", "said", "see", "the", "three", "to", "two", "up", "we",
        "where", "yellow", "you", "all", "am", "are", "at", "ate", "be", "black", "brown", "but",
        "came", "did", "do", "eat", "four", "get", "good", "have", "he", "into", "like", "must",
        "new", "no", "now", "on", "our", "out", "please", "pretty", "ran", "ride", "saw", "say",
        "she", "so", "soon", "that", "there", "they", "this", "too", "under", "want", "was",
        "well", "went", "what", "white", "who", "will", "with", "yes", "after", "again", "an",
        "any", "as", "ask", "by", "could", "every", "fly", "from", "give", "going", "had", "has",
        "her", "him", "his", "how", "just", "know", "let", "live", "may", "of", "old", "once",
        "open", "over", "put", "round", "some", "stop", "take", "thank", "them", "then", "think",
        "walk", "were", "when", "always", "around", "because", "been", "before", "best", "both",
        "buy", "call", "cold", "does", "don't", "fast", "first", "five", "found", "gave", "goes",
        "green", "its", "left", "made", "many", "off", "or", "pull", "read", "right", "sing",
        "sit", "sleep", "tell", "their", "these", "those", "upon", "us", "use", "very", "wash",
        "which", "why", "wish", "work", "would", "write", "your", "about", "better", "bring",
        "carry", "clean", "cut", "done", "draw", "drink", "eight", "fall", "far", "full", "got",
        "grow", "hold", "hot", "hurt", "if", "keep", "kind", "laugh", "light", "long", "much",
        "myself", "never", "only", "own", "pick", "seven", "shall", "show", "six", "small",
        "start", "ten", "today", "together", "try", "warm",
    ];

    return words;
}

/// Generate random words with given amount on given Language.
pub fn generate_words(lang: Lang, amount: u16) -> Words {
    let words: Words = match lang {
        Lang::English => en_words(),
    };

    let mut rng = rand::rng(); // Randomize selected words.
    let chosen_words: Words = words
        .choose_multiple(&mut rng, amount as usize)
        .cloned()
        .collect();

    return chosen_words;
}

pub fn stylize_word(word: Word, user_type: String) -> String {
    let mut styled_word: String = String::new(); // Create a new string for styled text.
    let word_chars: Vec<char> = word.chars().collect();
    let user_type_chars: Vec<char> = user_type.chars().collect();
    // Loop for characters and index in `word_chars`.
    for (index, ch) in word_chars.iter().enumerate() {
        let user_ch = user_type_chars.get(index);
        // If there is no typed char from user stylize it with Gray.
        if user_ch.is_none() {
            styled_word.push_str(&colorize(&ch.to_string(), Color::Gray));
            continue;
        }

        // If the user typed char and word char is same stylize it with Green.
        if user_ch.unwrap() == ch {
            styled_word.push_str(&colorize(&ch.to_string(), Color::Green));
            continue;
        }

        // If the user typed char and word char is different stylize it with Red.
        if user_ch.unwrap() != ch {
            styled_word.push_str(&colorize(&ch.to_string(), Color::Red));
        }
    }
    return styled_word;
}
