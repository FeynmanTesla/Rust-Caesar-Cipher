use std::borrow::Borrow;

use rand::Rng;

mod dictionary;

//TODO: fix bug that auto decryption works fine without punctuation but doesn't with (for example) full stops at the end of words.

static ALPHABET_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
static ALPHABET_UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
static LETTERS_IN_ALPHABET: i8 = 26;
static PASSABLE_PROPORTION_WORDS_IN_DICT: f32 = 0.9;

/// Find the output of the program (in the form of a string) for a given input.
/// Used by both the GUI and when getting output for the CLI.
/// Take params describing whether encrypting, choosing shift size automatically, the shift size (used if manual selection), and text input.
pub(crate) fn find_output(encrypting: bool, shift_size_automatic: bool, shift_size: f64, input: String) -> String {
    if encrypting {
        if shift_size_automatic {
            encrypt(gen_shift(), input.borrow())
        } else {
            encrypt(shift_size as i8, input.borrow())
        }
    } else {
        if shift_size_automatic {
            auto_decrypt(input.borrow())
        } else {
            decrypt(Some(shift_size as i8), input.borrow())
        }
    }
}

/// Encrypt plaintext into ciphertext.
/// Parameters are the plaintext and shift size.
/// Returns a String with the ciphertext.
fn encrypt(shift_value: i8, plaintext: &String) -> String {
    let ciphertext: String = shift_text(plaintext, shift_value, true).unwrap();
    format!("For the plaintext \"{}\", given a shift of {}, the Caesar ciphertext is \"{}\".", plaintext, shift_value, ciphertext)
}

/// Decrypt ciphertext into plaintext.
/// Parameters are the ciphertext and shift size.
/// Returns a String with the plaintext.
fn decrypt(shift_value: Option<i8>, ciphertext: &String) -> String {
    match shift_value {
        None => decrypt(Some(gen_shift()), ciphertext),
        Some(shift_value) => {
            let plaintext: String = shift_text(ciphertext, shift_value, false).unwrap();
            format!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift_value, plaintext)
        }
    }
}

/// Use a random number generator to generate a shift size to be applied.
/// Return the shift size as an i8.
fn gen_shift() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, LETTERS_IN_ALPHABET - 1)
}

/// Decrypt ciphertext into plaintext when the shift size is unknown.
/// Try all possible shift sizes, matching the words in results to a dictionary.
/// If enough match words then use that shift size and return the resulting plaintext. Otherwise, return a String explaining that the plaintext couldn't be found.
/// The sole parameter is the ciphertext.
/// Return the result in a String object.
fn auto_decrypt(ciphertext: &String) -> String {
    for shift in 1..LETTERS_IN_ALPHABET {
        match try_decrypt(shift, ciphertext, &dictionary::DICTIONARY_ARRAY) {
            Some(result) => {
                return result;
            }
            _ => {}
        }
    }
    format!("Failed to automatically decrypt the ciphertext \"{}\".", ciphertext)
}

/// Try to decrypt ciphertext into plaintext when the shift size is unknown.
/// Use a possible shift size and apply it to the ciphertext to find the possible plaintext.
/// If enough of the words in the resulting plaintext matches a dictionary, assume it's correct.
/// If enough match, return the resulting String object inside a Some() wrapper; else, return None.
/// Parameters are the shift size, ciphertext, and a vector holding the dictionary words.
/// Return an Option<String> with Some(String) if enough plaintext words were in the dictionary, otherwise return None.
fn try_decrypt(shift_value: i8, ciphertext: &String, dictionary_words: &[&'static str; 370104]) -> Option<String> {
    let possible_plaintext: String = shift_text(ciphertext, shift_value, false).unwrap();

    let words: Vec<String> = possible_plaintext.split(" ").map(|s: &str| String::from(s)).collect();
    let mut words_in_dict: i8 = 0;

    for word in &words {
        if dictionary_words.contains(&&**&word.to_lowercase()) { words_in_dict += 1; }
    }

    let is_passable_text: bool = words_in_dict as f32 / words.len() as f32 >= PASSABLE_PROPORTION_WORDS_IN_DICT;

    if is_passable_text {
        let res: String = format!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift_value, possible_plaintext);
        return Some(res);
    }

    None
}

/// Apply a Caesar shift to given text and return the result.
/// Can be applied "right or left" to encrypt plaintext into ciphertext or vice versa.
/// Return as an option<String> since the shift_word() method applied to each word can fail and so returns an option.
/// Given parameters of the start text, shift size, and if encrypting (not decrypting).
/// Return Some(String) if each word could be shifted, otherwse None.
fn shift_text(start: &String, shift: i8, encrypting: bool) -> Option<String> {
    let mut words: Vec<String> = start.split(" ").map(|s: &str| String::from(s)).collect();
    for i in 0..words.len() {
        match shift_word(&words[i], shift, encrypting) {
            None => {
                println!("Can't shift the input \"{}\".", start);
                return None;
            }
            Some(str) => {
                words[i] = str;
            }
        }
    }
    Some(words.join(" "))
}

/// Shift a word with a given shift size and direction (right/encrypt or left/decrypt).
/// Shift each character in the word in turn and then return Some(String) of the result if it works.
/// Shifting a character is done in shift_char(), which can fail: returning Option<char>.
/// Parameters are the starting string, shift size, and direction (right/encrypt or left/decrypt).
/// Return Some(String) from the concatenated shifted chars or None if a char shift fails.
fn shift_word(start: &String, shift: i8, right: bool) -> Option<String> {
    let mut chars: Vec<char> = vec![];
    for c in start.chars() {
        match shift_char(&c, shift, right) {
            None => {
                println!("Can't shift the word \"{}\".", start);
                return None;
            }
            Some(character) => {
                chars.push(character);
            }
        }
    }
    Some(chars.into_iter().collect())
}

/// Shift a character by a given amount and direction (right/encrypt or left/decrypt).
/// Look for the character in a fixed-size array holding the alphabet then move to an index according to the shift size and index, then return that character.
/// Since the search for the character in the alphabet array can fail - returning Option<usize>, the method can fail so returns Option<char>.
/// If the index of the character can be found, find the index of the char after the shift and return it wrapped in a Some().
/// If the index of the input character can't be found, return None.
/// Parameters are the starting character, shift size, and direction (right/encrypt or left/decrypt).
/// Return Some(char) from the shifted char if the initial index match works or None it fails.
fn shift_char(start: &char, mut shift: i8, right: bool) -> Option<char> {
    let alphabet: &[char; 26];

    if (!ALPHABET_UPPER.contains(start)) && (!ALPHABET_LOWER.contains(start)) {
        return Some(*start);
    }

    if ALPHABET_UPPER.contains(start) {
        alphabet = &ALPHABET_UPPER;
    } else {
        alphabet = &ALPHABET_LOWER;
    }

    if !right {
        shift *= -1;
    }

    let usize_index: Option<usize> = alphabet.iter().position(|char: &char| char == start);

    match usize_index {
        None => {
            println!("Can't shift the character \"{}\".", start);
            None
        }
        Some(usize_index) => {
            let mut index: i8 = usize_index as i8;
            if right {
                index = (index + shift) % alphabet.len() as i8;
            } else {
                index = (index + shift) % alphabet.len() as i8;
            }
            assert!(index > -26 && index < 26);
            if index < 0 {
                index += 26;
            }
            if index == 26 {
                index = 0;
            }
            assert!(index > -1 && index < 26);
            Some(alphabet[index as usize])
        }
    }
}