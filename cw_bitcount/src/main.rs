#[allow(unused)]
fn count_bits(n: i64) -> u32 {
    format!("{:b}", n).rmatches('1').count() as u32
}

fn count_bits2(n: i64) -> u32 {
    n.count_ones()
}

fn main() {
    println!("{}", count_bits2(1234));
}
