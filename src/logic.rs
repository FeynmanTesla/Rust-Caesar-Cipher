use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};

use druid::{Data, Lens};
use rand::Rng;

static ALPHABET_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
static ALPHABET_UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
static LETTERS_IN_ALPHABET: i8 = 26;
static PASSABLE_PROPORTION_WORDS_IN_DICT: f32 = 0.9;

#[derive(Clone, Data, Lens)]
pub(crate) struct AppState {
    encrypting: bool,
    shift_size_automatic: bool,
    shift_size: f64,
    input: String,
    output: String,
}

// TODO: separate logic and state stuff

pub(crate) fn get_initial_state() -> AppState {
    AppState {
        encrypting: false,
        shift_size_automatic: false,
        shift_size: 10.0,
        input: "".to_string(),
        output: "".to_string(),
    }
}

impl AppState {
    pub(crate) fn get_encrypting(&self) -> bool {
        self.encrypting
    }

    pub(crate) fn set_encrypting(&mut self, val: bool) {
        self.encrypting = val;
    }

    pub(crate) fn get_shift_size_automatic(&self) -> bool {
        self.shift_size_automatic
    }

    pub(crate) fn set_shift_size_automatic(&mut self, val: bool) {
        self.shift_size_automatic = val;
    }

    pub(crate) fn get_shift_size(&self) -> f64 {
        self.shift_size
    }

    pub(crate) fn get_output(&self) -> &str {
        &*self.output
    }

    pub(crate) fn update_output(&mut self) {
        self.output = find_output(self);
    }
}

pub(crate) fn find_output(state: &mut AppState) -> String {
    if state.encrypting {
        if state.shift_size_automatic {
            encrypt(gen_shift(), state.input.borrow())
        } else {
            encrypt(state.shift_size as i8, state.input.borrow())
        }
    } else {
        if state.shift_size_automatic {
            auto_decrypt(state.input.borrow())
        } else {
            decrypt(Some(state.shift_size as i8), state.input.borrow())
        }
    }
}

fn encrypt(shift_value: i8, plaintext: &String) -> String {
    let ciphertext: String = shift_text(plaintext, shift_value, true).unwrap();
    format!("For the plaintext \"{}\", given a shift of {}, the Caesar ciphertext is \"{}\".", plaintext, shift_value, ciphertext)
}

fn decrypt(shift_value: Option<i8>, ciphertext: &String) -> String {
    match shift_value {
        None => decrypt(Some(gen_shift()), ciphertext),
        Some(shift_value) => {
            let plaintext: String = shift_text(ciphertext, shift_value, false).unwrap();
            format!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift_value, plaintext)
        }
    }
}

fn gen_shift() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, LETTERS_IN_ALPHABET - 1)
}

fn auto_decrypt(ciphertext: &String) -> String {
    let dictionary_words: Vec<String> = get_dictionary_words();
    for shift in 1..LETTERS_IN_ALPHABET {
        match try_decrypt(shift, ciphertext, &dictionary_words) {
            Some(result) => {
                return result;
            }
            _ => {}
        }
    }
    format!("Failed to automatically decrypt the ciphertext \"{}\".", ciphertext)
}

fn try_decrypt(shift_value: i8, ciphertext: &String, dictionary_words: &Vec<String>) -> Option<String> {
    let possible_plaintext: String = shift_text(ciphertext, shift_value, false).unwrap();

    let words: Vec<String> = possible_plaintext.split(" ").map(|s: &str| String::from(s)).collect();
    let mut words_in_dict: i8 = 0;

    for word in &words {
        if dictionary_words.contains(&word.to_lowercase()) { words_in_dict += 1; }
    }

    let is_passable_text: bool = words_in_dict as f32 / words.len() as f32 >= PASSABLE_PROPORTION_WORDS_IN_DICT;

    if is_passable_text {
        let res: String = format!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift_value, possible_plaintext);
        return Some(res);
    }

    None
}

fn get_dictionary_words() -> Vec<String> {
    let file = File::open("src/dictionary.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

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