#![allow(unused)]

fn main() {
    let result = solution2(11);
    println!("Value: {}", result);
}

fn solution(num: i32) -> i32 {
    let mut result = 0;
    if result < 0 {
        return 0;
    }
    for i in 1..num {
        if i % 3 == 0 || i % 5 == 0 {
            result += i
        }
    }
    result
}

fn solution2(num: i32) -> i32 {
    (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
