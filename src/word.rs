use crate::utils::{colorize, Color};
use rand::prelude::IndexedRandom;
use rand::seq::IteratorRandom; // More appropriate random trait
use std::fmt;

/// Type alias for representing a word as a reference to a static string.
pub type Word = &'static str;
/// Type alias for representing a collection of words.
pub type Words = Vec<Word>;

/// Enum to represent supported languages.
pub enum Lang {
    English,
    Spanish,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Lang::English => "en",
            Lang::Spanish => "es",
        };
        write!(f, "{}", s)
    }
}

/// Returns the most used 120 English keywords.
/// This is a private function used internally by `generate_words`.
fn en_words() -> Words {
    vec![
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
    ]
}

/// Returns the most used 100 Spanish keywords.
fn es_words() -> Words {
    vec![
        "el",
        "la",
        "de",
        "que",
        "y",
        "a",
        "en",
        "un",
        "ser",
        "se",
        "no",
        "haber",
        "por",
        "con",
        "su",
        "para",
        "como",
        "estar",
        "tener",
        "le",
        "lo",
        "lo",
        "todo",
        "pero",
        "más",
        "hacer",
        "o",
        "poder",
        "decir",
        "este",
        "ir",
        "otro",
        "ese",
        "la",
        "si",
        "me",
        "ya",
        "ver",
        "porque",
        "dar",
        "cuando",
        "él",
        "muy",
        "sin",
        "vez",
        "mucho",
        "saber",
        "qué",
        "sobre",
        "mi",
        "alguno",
        "mismo",
        "yo",
        "también",
        "hasta",
        "año",
        "dos",
        "querer",
        "entre",
        "así",
        "primero",
        "desde",
        "grande",
        "eso",
        "ni",
        "nos",
        "llegar",
        "pasar",
        "tiempo",
        "ella",
        "sí",
        "día",
        "uno",
        "bien",
        "poco",
        "deber",
        "entonces",
        "poner",
        "cosa",
        "tanto",
        "hombre",
        "parecer",
        "nuestro",
        "tan",
        "donde",
        "ahora",
        "parte",
        "después",
        "vida",
        "quedar",
        "siempre",
        "creer",
        "hablar",
        "llevar",
        "dejar",
        "nada",
        "cada",
        "seguir",
        "menos",
        "nuevo",
        "encontrar",
    ]
}

/// Generate random words based on the selected language and amount.
pub fn generate_words(lang: &Lang, amount: u16) -> Words {
    let words = match lang {
        Lang::English => en_words(),
        Lang::Spanish => es_words(),
    };

    let mut rng = rand::thread_rng(); // Use a thread-local RNG.
    words
        .choose_multiple(&mut rng, amount as usize)
        .cloned()
        .collect()
}

/// Stylize a word by comparing it to the user-typed string.
pub fn stylize_word(word: Word, user_type: String) -> String {
    let word_chars: Vec<char> = word.chars().collect();
    let user_type_chars: Vec<char> = user_type.chars().collect();

    word_chars
        .iter()
        .enumerate()
        .map(|(index, &ch)| {
            let user_ch = user_type_chars.get(index);
            match user_ch {
                Some(&uc) if uc == ch => colorize(&ch.to_string(), Color::Green),
                Some(_) => colorize(&ch.to_string(), Color::Red),
                None => colorize(&ch.to_string(), Color::Gray),
            }
        })
        .collect::<String>()
}
