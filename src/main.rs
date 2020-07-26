use std::env;
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};

static ALPHABET_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
static ALPHABET_UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
static LETTERS_IN_ALPHABET: i32 = 26;
static PASSABLE_PROPORTION_WORDS_IN_DICT: f32 = 0.9;

// TODO: fix errors: convert i32 uses to u64; fix auto decrypting
// TODO: make unit tests
// TODO: make frontend with GTK

fn main() {
    let args: Vec<String> = env::args().collect();

    let encrypting: bool = &args[1] == "E";
    let shift : &String = &args[2];
    let raw_text : &String = &args[3..].join(" ");

    // args: <E for encrypt, D for decrypt> <x num to shift encrypting or 'A' for auto random num> <src text>

    // TODO: add improved error handling, inc. usage feedback

    if encrypting {
        if shift == "A" {
            encrypt(gen_shift(), raw_text);
        }
        else {
            encrypt((&shift).parse::<i32>().unwrap() as i32, raw_text);
        }
    }

    else {
        if shift == "A" {
            auto_decrypt(raw_text);
        }
        else {
            decrypt(Some((&shift).parse::<i32>().unwrap() as i32), raw_text);
        }
    }
}

fn encrypt(shift_value: i32, plaintext: &String) {
    let ciphertext: String = shift_text(plaintext, shift_value, true).unwrap();
    println!("For the plaintext \"{}\", given a shift of {}, the Caesar ciphertext is \"{}\".", plaintext, shift_value, ciphertext);
}

fn decrypt(shift_value: Option<i32>, ciphertext: &String) {
    match shift_value {
        None => decrypt(Some(gen_shift()), ciphertext),
        Some(shift_value) => {
            let plaintext: String = shift_text(ciphertext, shift_value, false).unwrap();
            println!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift_value, plaintext);
        }
    }
}

fn gen_shift() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, LETTERS_IN_ALPHABET - 1)
}

fn auto_decrypt(ciphertext: &String) {
    let dictionary_words: Vec<String> = get_dictionary_words();
    for shift in 1 .. LETTERS_IN_ALPHABET {
        if try_decrypt(shift, ciphertext, &dictionary_words) { return; }
    }
    println!("Failed to automatically decrypt the ciphertext \"{}\".", ciphertext);
}

fn try_decrypt(shift_value: i32, ciphertext: &String, dictionary_words: &Vec<String>) -> bool {
    let possible_plaintext: String = shift_text(ciphertext, shift_value, false).unwrap();

    let words: Vec<String> = possible_plaintext.split(" ").map(|s: &str| String::from(s)).collect();
    let mut words_in_dict: i32 = 0;

    for word in &words {
        if dictionary_words.contains(word) { words_in_dict += 1; }
    }

    let is_passable_text: bool = words_in_dict as f32 / words.len() as f32 >= PASSABLE_PROPORTION_WORDS_IN_DICT;

    if is_passable_text {
        println!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift_value, possible_plaintext);
    }

    is_passable_text
}

fn get_dictionary_words() -> Vec<String> {
    let file = File::open("src/dictionary.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn shift_text(start: &String, shift: i32, encrypting: bool) -> Option<String> {
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

fn shift_word(start: &String, shift: i32, right: bool) -> Option<String> {
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

fn shift_char(start: &char, mut shift: i32, right: bool) -> Option<char> {
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

    let usize_index: Option<usize> = alphabet.iter().position(|char : &char| char == start);

    match usize_index {
        None => {
            println!("Can't shift the character \"{}\".", start);
            None
        }
        Some(usize_index) => {
            let mut index: i32 = usize_index as i32;
            if right {
                index = (index + shift) % alphabet.len() as i32;
            } else {
                index = (index + shift) % alphabet.len() as i32;
            }
            Some(alphabet[index as usize])
        }
    }
}