fn make_upper_case(s: &str) -> String {
    s.to_uppercase().to_string()
}

fn main() {
    let s = String::from("world");
    println!("{} -> {}", s, make_upper_case(&s));
}
