use std::collections::HashMap;

fn sort_chars(s: &str) -> String {
    let mut char_sort = s.chars().collect::<Vec<char>>();
    char_sort.sort_unstable();
    char_sort.into_iter().collect::<String>()
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
