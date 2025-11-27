// "is2 Thi1s T4est 3a"  -->  "Thi1s is2 3a T4est"
// "4of Fo1r pe6ople g3ood th5e the2"  -->  "Fo1r the2 g3ood 4of th5e pe6ople"

use std::cmp::*;

fn compare_by_num(a: &String, b: &String) -> Ordering {
    let mut A = 0;
    let mut B = 0;
    for letter_a in a.chars() {
        if letter_a.is_numeric() {
            A = letter_a.to_digit(10).unwrap();
        }
    }
    for letter_b in b.chars() {
        if letter_b.is_numeric() {
            B = letter_b.to_digit(10).unwrap();
        }
    }
    let test = A.cmp(&B);
    test
}

fn main() {
    let text = String::from("is2 Thi1s T4est 3a");
    let mut vtxt = vec![];
    for i in text.split_whitespace() {
        vtxt.push(i.to_string());
    }

    vtxt.sort_by(compare_by_num);
    println!("{:?}",vtxt.join(" "));

    let text2 = String::from("4of Fo1r pe6ople g3ood th5e the2");
    let vtxt2: Vec<_> = text2.split_whitespace().map(String::from).collect();
    for i in vtxt2 {
        println!("{i}");
    }

}

/*
fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}
 */