pub fn increment_string2(s: &str) -> String {
    if s.is_empty() {
        return "1".to_string();
    }

    // Find the boundary between the non-digit prefix and the numeric suffix
    let mut split_index = s.len();
    for (i, c) in s.chars().rev().enumerate() {
        if !c.is_ascii_digit() {
            split_index = s.len() - i;
            break;
        }
    }

    // Split the string into prefix and numeric suffix
    let prefix = &s[..split_index];
    let number_part = &s[split_index..];

    if number_part.is_empty() {
        // No number at the end, just append "1"
        return format!("{}{}", prefix, 1);
    }

    // Parse the number, handling leading zeros
    let num_digits = number_part.len();
    let num_value: u64 = number_part.parse().unwrap_or(0);
    let incremented = num_value + 1;

    // Format with proper zero padding
    let incremented_str = format!("{:0width$}", incremented, width = num_digits);

    // Special case: if the incremented number has more digits than original,
    // we need to handle overflow of leading zeros (e.g., 099 -> 100)
    if incremented_str.len() > num_digits {
        format!("{}{}", prefix, incremented)
    } else {
        format!("{}{}", prefix, incremented_str)
    }
}

use std::iter;
fn increment_string3(s: &str) -> String {
    let num = s
        .chars()
        .rev()
        .take_while(|x| x.is_numeric())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    if num.is_empty() {
        return format!("{}1", s);
    }

    let num_length = num.len();
    let mut numeric = num.parse::<u128>().unwrap_or(0);
    numeric += 1;
    let numeric_string = numeric.to_string();

    // Special case: if the incremented number has more digits than original,
    // we need to handle overflow of leading zeros (e.g., 099 -> 100)
    if numeric_string.len() > num_length {
        // Number of digits increased (e.g., 99 -> 100), no padding needed
        format!(
            "{}{}",
            s.chars().take(s.len() - num_length).collect::<String>(),
            numeric_string
        )
    } else {
        // Number of digits stayed the same, pad with leading zeros
        let difference = num_length - numeric_string.len();
        let num = iter::repeat('0').take(difference).collect::<String>() + &numeric_string;

        format!(
            "{}{}",
            s.chars().take(s.len() - num_length).collect::<String>(),
            num
        )
    }
}

fn increment_big_number(num: &str) -> String {
    let mut digits: Vec<u8> = num.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    let mut i = digits.len() as i32 - 1;
    let mut carry = 1;

    while i >= 0 && carry > 0 {
        let idx = i as usize;
        let sum = digits[idx] + carry;
        digits[idx] = sum % 10;
        carry = sum / 10;
        i -= 1;
    }

    if carry > 0 {
        // Prepend the carry
        let mut result = vec![1];
        result.extend(digits);
        result.iter().map(|&d| (b'0' + d) as char).collect()
    } else {
        digits.iter().map(|&d| (b'0' + d) as char).collect()
    }
}

fn increment_string(s: &str) -> String {
    // Find where the numeric suffix starts
    let split_point = s
        .chars()
        .rev()
        .position(|c| !c.is_ascii_digit())
        .map(|pos| s.len() - pos)
        .unwrap_or(0);

    let (prefix, number_part) = s.split_at(split_point);

    if number_part.is_empty() {
        format!("{}1", prefix)
    } else {
        let incremented = increment_big_number(number_part);
        format!("{}{}", prefix, incremented)
    }
}

fn main() {
    println!("{}", increment_string("foobar001"));
    println!("{}", increment_string("foo"));
    println!("{}", increment_string("foobar099"));
    println!("{}", increment_string("foobar99"));
    println!("{}", increment_string("foobar1"));
    println!("{}", increment_string(""));
}
