use std::collections::{HashMap, HashSet};

fn sort_chars(s: &str) -> String {
    //let char_sort = s.to_lowercase().chars().collect::<HashSet<char>>();
    //let mut char_sort = char_sort.into_iter().collect::<Vec<char>>();

    let mut char_sort = s
        .chars()
        .filter(|&c| c != ' ')
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();
    char_sort.sort_unstable(); // sort in place, does not return vector
    char_sort.into_iter().collect::<String>() // return to String
}

pub fn find_uniq<'a>(arr: &'a [&str]) -> &'a str {
    let freq = arr.iter().fold(HashMap::new(), |mut acc, x| {
        let key = sort_chars(x);
        *acc.entry(key).or_insert(0) += 1;
        acc
    });

    arr.iter()
        .find(|&&x| {
            let key = sort_chars(x);
            freq[&key] == 1
        })
        .unwrap()
}

fn main() {
    println!(
        "{}",
        find_uniq(&["abc", "acb", "bac", "foo", "bca", "cab", "cba"])
    ); // foo
}
