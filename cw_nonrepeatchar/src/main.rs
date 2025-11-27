#[allow(unused)]
use std::collections::HashMap;

pub fn first_non_repeating(s: &str) -> Option<char> {
    let char_frequency = s
        .to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut acc, char| {
            *acc.entry(char).or_insert(0) += 1;
            acc
        });
    for i in s.chars() {
        let test_char = char_frequency.get(&i.to_ascii_lowercase()).unwrap();
        if *test_char == 1 {
            return Some(i);
        }
    }
    None
}

fn main() {
    println!("First: {:?}", first_non_repeating("moonmEn"));
}
