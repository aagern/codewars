#![allow(unused)]

// Complete the solution so that it splits the string into pairs of two characters.
// If the string contains an odd number of characters then it should replace the missing second character
// of the final pair with an underscore ('_').

// * 'abc' =>  ['ab', 'c_']
// * 'abcdef' => ['ab', 'cd', 'ef']

fn main() {
    let r = solution("abcdef");
    println!("Result: {:?}", r);
}

#[allow(dead_code)]
fn solution(s: &str) -> Vec<String> {
    let result = vec!["ab".to_string(), "cd".to_string()];
    result
}
