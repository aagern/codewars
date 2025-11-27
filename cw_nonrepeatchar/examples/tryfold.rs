fn main() {
    let strings = ["1", "2", "3", "4", "5"];
    // Convert strings to numbers and sum them
    let sum: Result<i32, _> = strings
        .iter()
        .try_fold(0, |acc, &s| match s.parse::<i32>() {
            Ok(num) => Ok(acc + num),
            Err(e) => Err(e),
        });

    match sum {
        Ok(total) => println!("Total: {}", total), // 15
        Err(e) => println!("Parse error: {}", e),
    }
}
