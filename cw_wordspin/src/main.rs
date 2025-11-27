#![allow(unused)]
// "Hey fellow warriors"  --> "Hey wollef sroirraw"
// "This is a test        --> "This is a test"
// "This is another test" --> "This is rehtona test"

fn main() {
    let text = String::from("Hey fellow warriors");
    let result = spin_words2(&text);
    println!("Text: {}; Spinned: {};", text, result);
}

fn spin_words(words: &str) -> String {
    let mut result_vec: Vec<String> = Vec::new();
    for i in words.split_whitespace() {
        if i.chars().count() >= 5 {
            result_vec.push(i.chars().rev().collect::<String>());
        } else {
            result_vec.push(i.to_string());
        }
    }
    result_vec.join(" ")
}

fn spin_words2(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .map(|word| match word.chars().count() >= 5 {
            true => word.chars().rev().collect(),
            false => word.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}
