#![allow(unused)]

fn main() {
    let vowels_count = get_count2("abracadabra");
    println!("Value: {}", vowels_count);
}

fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    for letter in string.chars() {
        if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u' {
            vowels_count += 1;
        }
    }

    vowels_count
}

fn get_count2(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    string.chars().filter(|&l| "aeiou".contains(l)).count()
}
