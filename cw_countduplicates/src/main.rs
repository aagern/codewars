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

fn main() {
    println!("{}", count_duplicates("aabBcde"));
}
