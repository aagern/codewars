// fn solution(s: &str) -> String {
//     let mut result = String::new();
//     for c in s.chars() {
//         if c.is_ascii_uppercase() {
//             result.push(' ');
//         }
//         result.push(c);
//     }
//     result
// }

use regex::Regex;
fn solution(s: &str) -> String {
    let re = Regex::new(r"[A-Z]").unwrap(); // находим заглавную букву
    re.replace_all(s, " $0").to_string() // добавляем перед ней пробел
} // camelCasingTest -> camel Casing Test

fn main() {
    println!("{}", solution("camelCasing"));
    println!("{}", solution("camelCasingTest"));
}
