#![allow(unused)]
use itertools::Itertools;

// Bad solution, complexity O(n!)
fn next_bigger_number2(n: u64) -> Option<u64> {
    let candidate = n.to_string().chars().collect::<Vec<char>>();

    let mut numbers: Vec<u64> = candidate
        .iter()
        .permutations(candidate.len())
        .unique()
        .filter_map(|perm| {
            perm.iter()
                .map(|&&c| c.to_digit(10).unwrap().to_string())
                .collect::<String>()
                .parse::<u64>()
                .ok()
        })
        .filter(|&x| x > n)
        .collect();
    numbers.sort();

    numbers.into_iter().next()
}

// Good solution, complexity O(n)
fn next_bigger_number(n: u64) -> Option<u64> {
    let mut digits: Vec<char> = n.to_string().chars().collect();

    // Find the first digit from the right that is smaller than the digit to its right
    let mut i = digits.len() - 1;
    while i > 0 && digits[i - 1] >= digits[i] {
        i -= 1;
    }

    // If no such digit exists, the number cannot be rearranged to form a bigger number
    if i == 0 {
        return None;
    }

    // digits[i-1] is the pivot
    let pivot = digits[i - 1];

    // Find the smallest digit to the right of the pivot that is greater than the pivot
    let mut j = digits.len() - 1;
    while digits[j] <= pivot {
        j -= 1;
    }

    // Swap the pivot with this digit
    digits.swap(i - 1, j);

    // Reverse the digits to the right of the original pivot position
    digits[i..].reverse();

    // Convert back to u64
    let result_str: String = digits.into_iter().collect();
    result_str.parse().ok()
}

fn main() {
    println!("{:?}", next_bigger_number(12)); // Some(21)
    println!("{:?}", next_bigger_number(513)); // Some(531)
    println!("{:?}", next_bigger_number(2017)); // Some(2071)
    println!("{:?}", next_bigger_number(9)); // None
    println!("{:?}", next_bigger_number(111)); // None
    println!("{:?}", next_bigger_number(531)); // None
    println!("{:?}", next_bigger_number(1234)); // Some(1243)
    println!("{:?}", next_bigger_number(414)); // Some(441)
    println!("{:?}", next_bigger_number(144)); // Some(414)
}
