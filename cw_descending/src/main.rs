use itertools::Itertools;
//Input: 42145 Output: 54421
//Input: 145263 Output: 654321
//Input: 123456789 Output: 987654321

fn main() {
    let n = 123456789;
    let res = descending_order(n);
    println!("Value: {}, Descending: {}", n, res);

    let text = "Hello world";
    let text_sorted = text.chars().sorted_unstable().rev().collect::<String>();
    println!("Text: {}, Sorted Text: {}", text, text_sorted);
}

fn descending_order(x: u64) -> u64 {
    let x_vec = x.to_string();
    let x_vec_sorted = x_vec.chars().sorted_unstable().rev().collect::<String>();
    x_vec_sorted.parse::<u64>().unwrap()
}
