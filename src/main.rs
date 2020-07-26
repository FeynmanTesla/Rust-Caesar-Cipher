use std::env;
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufRead};

const ALPHABET_LOWER: [char; 26] = "abcdefghijklmnopqrstuvwxyz".chars().collect();
const ALPHABET_UPPER: [char; 26] = "ABCDEFGIJKLMNOPQRSTUVWXYZ".chars().collect();
const LETTERS_IN_ALPHABET: i32 = 26;
const PASSABLE_PROPORTION_WORDS_IN_DICT: f32 = 0.9;

// TODO: fix errors: don't shift spaces; shift against alphabet not encoding values; allow double quotes; ensure auto decrypting doesnt just take first value
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

fn encrypt(shift: i32, plaintext: &String) {
    let ciphertext: String = plaintext.chars().map(|c| (c as u8 + shift as u8) as char).collect::<String>();
    println!("For the plaintext \"{}\", given a shift of {}, the Caesar ciphertext is \"{}\".", plaintext, shift, ciphertext);
}

fn decrypt(shift: Option<i32>, ciphertext: &String) {
    match shift {
        None => decrypt(Some(gen_shift()), ciphertext),
        Some(shift) => {
            let plaintext: String = ciphertext.chars().map(|c| (c as u8 - shift as u8) as char).collect::<String>();
            println!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift, plaintext);
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
}

fn try_decrypt(shift: i32, ciphertext: &String, dictionary_words: &Vec<String>) -> bool {
    let possible_plaintext: String = ciphertext.chars().map(|c| (c as u8 - shift as u8) as char).collect::<String>();
    if !possible_plaintext.contains(" ") { return false; }

    let words: Vec<String> = possible_plaintext.split(" ").map(|s: &str| String::from(s)).collect();
    let mut words_in_dict: i32 = 0;

    for word in &words {
        if dictionary_words.contains(word) { words_in_dict+= 1; }
    }

    let is_passable_text: bool = words_in_dict as f32 / words.len() as f32 >= PASSABLE_PROPORTION_WORDS_IN_DICT;

    if is_passable_text {
        println!("For the ciphertext \"{}\", reversing a Caesar cipher shift of {}, the plaintext is \"{}\".", ciphertext, shift, possible_plaintext);
    }

    return is_passable_text;
}

fn get_dictionary_words() -> Vec<String> {
    let file = File::open("dictionary.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}