use num::bigint::BigUint;
use std::collections::HashMap;

// AI-created content, not fully functional solution!
// Does not pass tests in Codewars, because uses additional libs.
fn uniqcount(s: &str) -> BigUint {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        if ch.is_ascii_alphabetic() {
            *counts.entry(ch.to_ascii_uppercase()).or_insert(0u32) += 1;
        }
    }

    let n = s.len() as u32;

    // Compute directly: multiply range [1..n], canceling denominator factors
    // Use cancellation method to avoid huge intermediates

    // Create a vector of factors to multiply
    let mut factors: Vec<u32> = (1..=n).collect();

    // Cancel denominator factors
    for &count in counts.values() {
        if count > 1 {
            for denom_factor in 2..=count {
                // Try to cancel denom_factor with one of the remaining numerator factors
                for num_factor in factors.iter_mut() {
                    if *num_factor % denom_factor == 0 {
                        *num_factor /= denom_factor;
                        break;
                    }
                }
            }
        }
    }

    // Multiply remaining factors
    let mut result = BigUint::from(1u32);
    for &factor in &factors {
        if factor > 1 {
            result *= BigUint::from(factor);
        }
    }

    result
}

fn main() {
    // Test cases
    println!("uniqcount(\"AB\") = {}", uniqcount("AB")); // 2
    println!("uniqcount(\"ABC\") = {}", uniqcount("ABC")); // 6
    println!("uniqcount(\"ABA\") = {}", uniqcount("ABA")); // 3
    println!("uniqcount(\"ABBb\") = {}", uniqcount("ABBb")); // 4
    println!("uniqcount(\"AbcD\") = {}", uniqcount("AbcD")); // 24

    // Test with longer strings
    let test = "MISSISSIPPI";
    println!("uniqcount(\"{}\") = {}", test, uniqcount(test)); // 34650

    // Your failing tests
    let big1 = "ABcDEFgHIJbaslidbailsbdilasbdkanmsdklhkbHSJKHVDASH";
    println!("uniqcount(big1) = {}", uniqcount(big1));

    let big2 = "ntBjVSSICYZPMkXhmuRmvApXzvQXhMrCBVBsrnTYpAvBeqmrZwIdNW";
    println!("uniqcount(big2) = {}", uniqcount(big2));
}
