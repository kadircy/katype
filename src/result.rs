pub type Results = (f32, f32, f32);
//                  WPM, ACC, CONSISTENCY

pub fn calculate_result(words: &Vec<&str>, user_words: &Vec<&str>, duration: u16) -> Results {
    // Calculate WPM (Words Per Minute).
    let num_words_typed = user_words.len();
    let duration_minutes = duration as f32 / 60.0; // Convert duration from seconds to minutes.
    let wpm = num_words_typed as f32 / duration_minutes;

    // Calculate accuracy
    let correct_words = words
        .iter()
        .zip(user_words.iter())
        .filter(|(w1, w2)| w1 == w2)
        .count();
    let accuracy = if words.is_empty() {
        0.0
    } else {
        (correct_words as f32 / words.len() as f32) * 100.0
    };

    // Calculate consistency (count consecutive correct words from the beginning)
    let mut consistency = 0;
    for (w1, w2) in words.iter().zip(user_words.iter()) {
        if w1 == w2 {
            consistency += 1;
        } else {
            break; // Stop counting once we encounter the first incorrect word
        }
    }
    let consistency_percentage = if words.is_empty() {
        0.0
    } else {
        (consistency as f32 / words.len() as f32) * 100.0
    };

    return (
        wpm.round(),
        accuracy.round(),
        consistency_percentage.round(),
    );
}
