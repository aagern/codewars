#![allow(unused)]

// create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890")

fn main() {
    let result = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    println!("Phone: {}", result);
}

#[allow(dead_code)]
fn create_phone_number(numbers: &[u32]) -> String {
    format!(
        "({}{}{}) {}{}{}-{}{}{}{}",
        numbers[0],
        numbers[1],
        numbers[2],
        numbers[3],
        numbers[4],
        numbers[5],
        numbers[6],
        numbers[7],
        numbers[8],
        numbers[9]
    )
}
