use crate::word::Words;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json;

pub fn generate_code(words: &Words) -> String {
    let encoded_json = serde_json::to_string(words).expect("Invalid format of JSON in Words.");
    let base64 = URL_SAFE.encode(encoded_json);
    return base64;
}

pub fn resolve_code(code: &str) -> Words {
    // Decode the base64 string into bytes
    let decoded_bytes = URL_SAFE
        .decode(code)
        .expect("An error occurred while decoding code from Base64 format.");

    // Convert the bytes to a UTF-8 string
    let decoded_string =
        String::from_utf8(decoded_bytes).expect("Unable to translate u8 bytes to UTF-8 string.");

    // Ensure the decoded string is valid for the `'static` lifetime (using `Box<str>`)
    let static_str: &'static str = Box::leak(decoded_string.into_boxed_str());

    // Deserialize the decoded string into the `Words` type by borrowing `static_str`
    let decoded_words: Words =
        serde_json::from_str(&static_str).expect("Unable to decode words from string.");

    return decoded_words;
}
