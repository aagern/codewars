fn reducer(n: i64) -> i64 {
    let mut accum: i64 = 0;
    const RADIX: u32 = 10;

    for i in n.to_string().chars() {
        accum += i.to_digit(RADIX).unwrap() as i64;
    }
    accum
}

fn digital_root(n: i64) -> i64 {
    let mut root = n;
    while root / 10 > 0 {
        root = reducer(root);
    }
    root
}

fn main() {
    println!("{}", digital_root(493193));
}
