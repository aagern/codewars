fn high(input: &str) -> &str {
    input
        .split(' ')
        .rev() // for max_by_key(): if equal words, it returns the last, but task states to return the first
        .max_by_key(|word| word.chars().map(|letter| letter as u32 - 96).sum::<u32>())
        .unwrap()
}

fn main() {
    println!("{}", high("man i need a taxi up to ubud")); // taxi
    println!("{}", high("what time are we climbing up the volcano")); // volcano
    println!("{}", high("take me to semynak")); // semynak
    println!("{}", high("b aa")); // b - the first word in sentence is returned!
}
