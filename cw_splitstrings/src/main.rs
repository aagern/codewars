fn solution1(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();

    // Process complete pairs
    for chunk in chars.chunks_exact(2) {
        result.push(chunk.iter().collect());
    }

    // Handle the last odd character if any
    let remainder = chars.len() % 2;
    if remainder == 1 {
        let last_char = chars[chars.len() - 1];
        result.push(format!("{}_", last_char));
    }

    result
}

fn solution2(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();

    // Add underscore if odd number of characters
    if !chars.len().is_multiple_of(2) {
        chars.push('_');
    }

    // Split into pairs
    chars
        .chunks(2)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn solution3(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c1) = chars.next() {
        let c2 = chars.next().unwrap_or('_');
        result.push(format!("{}{}", c1, c2));
    }

    result
}

fn main() {
    println!("{:?}", solution1("abc"));
    println!("{:?}", solution2("abcdef"));
    println!("{:?}", solution3("abcdefg"));
}
