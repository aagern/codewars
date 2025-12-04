use std::time::Instant;

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn array_diff2<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.retain(|x| !b.contains(x));
    a
}

fn main() {
    let start = Instant::now();
    println!("{:?}", array_diff(vec![1, 2, 2], vec![1]));
    let duration_a = start.elapsed();

    let start = Instant::now();
    println!("{:?}", array_diff2(vec![1, 2, 2], vec![1]));
    let duration_b = start.elapsed();

    println!("Function diff1: {:?}", duration_a);
    println!("Function diff2: {:?}", duration_b);

    //Function array_diff1: 653.89µs
    //Function array_diff2: 3.93µs => x200 faster!
}
