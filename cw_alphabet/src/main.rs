#![allow(unused)]
use std::collections::HashMap;

fn main() {
    let result = alphabet_position2(&"The sunset sets at twelve o' clock.".to_string());
    println!("Result: {}", result);
}

#[allow(dead_code)]
fn alphabet_position(text: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut alpha_map: HashMap<char, String> = HashMap::new();
    let mut result = String::new();
    let mut i = 1u16; // enumerate() only works from 0
    for c in alphabet.chars() {
        alpha_map.insert(c, i.to_string());
        i += 1;
    }
    for n in text.to_lowercase().chars() {
        result.push_str(alpha_map.get(&n).unwrap_or(&"".to_string()));
        result.push_str(" ");
    }
    result
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

const CHAR_OFFSET: u8 = 96;
fn alphabet_position2(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c as u8 - CHAR_OFFSET).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
