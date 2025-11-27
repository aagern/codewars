#![allow(unused)]

const ALPHABET: &str = &"abcdefghijklmnopqrstuvwxyz";

fn main() {
    let result = is_pangram("The quick, brown fox jumps over the lazy dog!");
    println!("Pangram? {}", result);
}

fn is_pangram(s: &str) -> bool {
    for i in ALPHABET.chars() {
        if s.to_lowercase().contains(i) {
            continue;
        } else {
            return false;
        }
    }
    return true;
}
