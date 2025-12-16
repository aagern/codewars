use std::collections::HashMap;
fn find_uniq(arr: &[f64]) -> f64 {
    let freq = arr.iter().fold(HashMap::new(), |mut acc, f64| {
        let bits = f64.to_bits();
        *acc.entry(bits).or_insert(0) += 1;
        acc
    });

    *arr.iter()
        .find(|f64| freq.get(&f64.to_bits()).unwrap() == &1)
        .unwrap()
}

fn main() {
    println!("{}", find_uniq(&[1.0, 1.0, 1.0, 2.0, 1.0, 1.0]));
}
