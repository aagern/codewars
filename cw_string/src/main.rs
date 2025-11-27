fn reverse_words(str: &str) -> String {
    let mut result = String::new();
    let mut word = String::from("");
    let mut space = String::from("");

    for (j,i) in str.chars().enumerate() {
        if i != ' ' {
            result.push_str(space.as_str());
            word.push_str(i.to_string().as_str());
            space = "".to_string();
        } else {
            result.push_str(word.chars().rev().collect::<String>().as_str());
            word = "".to_string();
            space.push_str(i.to_string().as_str());
        }
        if j == (str.len()-1) {
            result.push_str(space.as_str());
            result.push_str(word.chars().rev().collect::<String>().as_str());
        }
    }
    result
}

fn reverse_words_split(str: &str) -> String {
    str.to_string().split(" ").map(|x| x.chars().rev().collect::<String>()).collect::<Vec<String>>().join(" ")
}

fn main() {
    let word: &str = "The   quick brown fox jumps over the lazy dog.";
    println!("{}",reverse_words_split(&word));
}
