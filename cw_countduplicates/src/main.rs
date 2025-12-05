use itertools::Itertools;
use std::collections::HashMap;
fn count_duplicates(text: &str) -> u32 {
    let mut accum: u32 = 0;
    let mut symbol_count: HashMap<char, u32> = HashMap::new();

    // For each new character in the text create a key with value = 1
    for key in text.to_lowercase().chars() {
        *symbol_count.entry(key).or_insert(0) += 1;
    }

    // Count the number of keys with value > 1
    for (_, v) in symbol_count {
        if v > 1 {
            accum += 1;
        }
    }
    accum
}

fn count_duplicates2(text: &str) -> u32 {
    text.to_lowercase()
        .chars()
        .counts() // replaces symbol_count in fn above
        .values() // receives HashMap and returns its' values
        .filter(|&&i| i > 1) // filters values > 1
        .count() as u32
}

fn main() {
    println!("{}", count_duplicates("aabBcde"));
    println!("{}", count_duplicates2("aabBcde"));
}
