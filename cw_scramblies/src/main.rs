use itertools::Itertools;
use std::collections::HashMap;
pub fn scramble(s1: &str, s2: &str) -> bool {
    let mut s1_freq = s1.chars().fold(HashMap::new(), |mut acc, char| {
        *acc.entry(char).or_insert(0) += 1;
        acc
    });

    for c in s2.chars() {
        let count = s1_freq.entry(c).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    true
}

pub fn scramble2(s1: &str, s2: &str) -> bool {
    let c1 = s1.chars().counts();
    // 1. s1.chars() creates an iterator over the characters of s1
    // 2. .counts() (cargo add itertools) counts the frequency of each character
    //    It returns a HashMap<char, usize> where keys are characters and
    //    values are how many times each character appears in s1
    let c2 = s2.chars().counts();
    // Same as above, but for s2 - creates a frequency map for s2 characters

    c2.iter().all(|(k, v)| c1.get(k).unwrap_or(&0) >= v)
    // 1. c2.iter() - iterates over key-value pairs in c2 HashMap
    // 2. .all(...) - returns true if ALL iterations satisfy the condition
    // 3. For each (k, v) pair from c2:
    //    - k is a character from s2
    //    - v is how many times that character appears in s2
    //    - c1.get(k) tries to get the count of character k from s1's frequency map
    //    - .unwrap_or(&0) returns the count if found, or 0 if not found
    //    - >= v checks if s1 has at least v occurrences of character k
    //    The condition is: s1's count of character k >= s2's count of character k
}

fn main() {
    println!("{}", scramble("rkqodlw", "world"));
    println!("{}", scramble("katas", "steak"));
}
