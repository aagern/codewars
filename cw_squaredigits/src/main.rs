fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|x| {
            let mut x = x.to_digit(10).unwrap() as u64;
            x *= x;
            x
        })
        .collect::<Vec<u64>>()
        .iter()
        .map(|&d| d.to_string())
        .collect::<String>()
        .parse::<u64>()
        .ok()
        .unwrap()
}

fn main() {
    println!("{}", square_digits(9119));
}
