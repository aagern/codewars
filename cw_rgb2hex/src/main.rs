fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        r.clamp(0, 255),
        g.clamp(0, 255),
        b.clamp(0, 255)
    )
}

fn rgb2(r: i32, g: i32, b: i32) -> String {
    [r, g, b]
        .iter()
        .map(|&x| x.clamp(0, 255))
        .map(|x| format!("{:02X}", x))
        .collect()
}

fn main() {
    println!("{}", rgb(0, 0, 0)); // 000000
    println!("{}", rgb(1, 2, 3)); // 010203
    println!("{}", rgb(255, 255, 255)); // FFFFFF
    println!("{}", rgb2(0, 0, 300)); // 0000FF
    println!("{}", rgb2(-20, 275, 125)); // 00FF7D
}
