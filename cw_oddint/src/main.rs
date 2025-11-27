#![allow(unused)]
//Given an array of integers, find the one that appears an odd number of times.
//There will always be only one integer that appears an odd number of times.

// [7] should return 7, because it occurs 1 time (which is odd).
// [0] should return 0, because it occurs 1 time (which is odd).
// [1,1,2] should return 2, because it occurs 1 time (which is odd).
// [0,1,0,1,0] should return 0, because it occurs 3 times (which is odd).
// [1,2,2,3,3,3,4,3,3,3,2,2,1] should return 4, because it appears 1 time (which is odd).

use std::collections::HashMap;

fn main() {
    let odd = find_odd(&vec![
        20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5,
    ]);
    println!("Odd integer: {}", odd);
}

fn find_odd(arr: &[i32]) -> i32 {
    let mut freqs: HashMap<i32, u32> = HashMap::new();
    for x in arr {
        *freqs.entry(*x).or_default() += 1;
    }
    for (k, v) in &freqs {
        if v % 2 != 0 {
            return *k;
        }
    }
    -1
}
