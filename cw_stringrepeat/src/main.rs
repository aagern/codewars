fn repeat_str(src: &str, count: usize) -> String {
    //std::iter::repeat_n(src, count).collect::<String>()
    src.repeat(count)
}

fn main() {
    println!("{}", repeat_str("I", 6));
}
