#![allow(unused)]

// dotest("the_stealth_warrior", "theStealthWarrior");
// dotest("The-Stealth-Warrior", "TheStealthWarrior");
// dotest("A-B-C", "ABC");

fn main() {
    println!("Hello, world!");
    let text = String::from("the_stealth_warrior");
    let result = to_camel_case2(&text);
    println!("Result: {}", result);

    let text2 = String::from("the_stealth_warrior");
    let parts = text2.split(['-', '_']);
    for part in parts {
        println!("{}", part);
    }
}

fn to_camel_case2(text: &str) -> String {
    text.split(&['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
            0 => w.to_string(),
            _ => w[..1].to_uppercase() + &w[1..],
        })
        .collect()
}

fn to_camel_case(text: &str) -> String {
    let mut result = String::from(text);
    let mut finres = String::new();
    if text.len() == 0 {
        return text.to_string();
    }
    if text.contains("-") {
        result = splitter("-", text).to_string();
    }
    if text.contains("_") {
        result = splitter("_", text).to_string();
    }

    let char_vec: Vec<char> = text.chars().collect();
    if char_vec[0].is_lowercase() {
        finres.push_str(&text[0..1]);
        finres.push_str(&result[1..]);
        return finres;
    } else {
        return result;
    }
}

fn splitter(pattern: &str, text: &str) -> String {
    let parts = text.split(pattern);
    let mut result = String::new();
    for part in parts {
        result.push_str(&part[0..1].to_uppercase());
        result.push_str(&part[1..]);
    }
    result
}
