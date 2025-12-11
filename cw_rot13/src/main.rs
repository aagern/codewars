fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'a'..='m' | 'A'..='M' => (c as u8 + 13) as char,
            'n'..='z' | 'N'..='Z' => (c as u8 - 13) as char,
            _ => c,
        })
        .collect()
}

fn main() {
    println!("{}", rot13("Test")); // Grfg

    println!("{}", 'a' as u8);
    println!("{}", 97 as char);
    println!("{:?}", std::char::from_u32(98).unwrap());
}
