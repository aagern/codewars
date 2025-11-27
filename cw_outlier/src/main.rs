fn find_outlier(values: &[i32]) -> i32 {
    let splitter = values
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut evens, mut odds), &x| {
            if x % 2 == 0 {
                evens.push(x);
            } else {
                odds.push(x);
            }
            (evens, odds)
        });
    if splitter.0.len() == 1 {
        splitter.0[0]
    } else {
        splitter.1[0]
    }
}

fn main() {
    println!(
        "{}",
        find_outlier(&[
            1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781, 206847684
        ])
    );
}
