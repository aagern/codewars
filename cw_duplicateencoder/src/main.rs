use std::collections::HashMap;
#[allow(unused)]
fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    let char_frequency = word.chars().fold(HashMap::new(), |mut acc, char| {
        *acc.entry(char).or_insert(0) += 1;
        acc
    });
    word.chars()
        .map(|x| match char_frequency.get(&x).unwrap() {
            1 => '(',
            _ => ')',
        })
        .collect()
}
#[allow(unused)]
fn duplicate_encode2(word: &str) -> String {
    let s = String::from(word).to_lowercase();
    s.chars()
        .map(|c| if s.find(c) == s.rfind(c) { '(' } else { ')' })
        .collect()
}

fn main() {
    println!("{}", duplicate_encode("rEcede"));
}
