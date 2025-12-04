use std::collections::BTreeSet;
use std::time::Instant;

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn array_diff2<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.retain(|x| !b.contains(x));
    a
}

fn array_diff3<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: PartialEq + Ord + Clone,
{
    let removal_list: BTreeSet<_> = b.iter().cloned().collect();
    a.into_iter()
        .filter(|x| !removal_list.contains(x))
        .collect()
}

fn main() {
    let start = Instant::now();
    println!("{:?}", array_diff(vec![1, 2, 2], vec![1]));
    let duration_a = start.elapsed();

    let start = Instant::now();
    println!("{:?}", array_diff2(vec![1, 2, 2], vec![1]));
    let duration_b = start.elapsed();

    let start = Instant::now();
    println!("{:?}", array_diff3(vec![1, 2, 2], vec![1]));
    let duration_c = start.elapsed();

    println!("Function diff1: {:?}", duration_a);
    println!("Function diff2: {:?}", duration_b);
    println!("Function diff3: {:?}", duration_c);

    /* cargo run --release
    Function diff1: 342.077µs
    Function diff2: 2.489µs
    Function diff3: 2.37µs
    */
}
