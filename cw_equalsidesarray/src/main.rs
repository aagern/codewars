fn find_even_index2(arr: &[i32]) -> Option<usize> {
    for (index, _) in arr.iter().enumerate() {
        let left: i32 = arr.iter().take(index).sum::<i32>();
        let right: i32 = arr.iter().skip(index).sum::<i32>();
        println!("left={left},right={right}");
        if left == right {
            return Some(index);
        }
    }
    None
}

fn find_even_index(arr: &[i32]) -> Option<usize> {
    let total_sum: i32 = arr.iter().sum();
    let mut left_sum = 0;

    for (i, _) in arr.iter().enumerate() {
        let right_sum = total_sum - left_sum - arr[i];
        if left_sum == right_sum {
            return Some(i);
        }
        left_sum += arr[i];
    }

    None
}

fn main() {
    println!("{:?}", find_even_index(&[1, 2, 3, 4, 3, 2, 1])); // Some(3)
    println!("{:?}", find_even_index(&[1, 100, 50, -51, 1, 1])); // Some(1)
    println!("{:?}", find_even_index(&[1, 2, 3, 4, 5, 6])); // None
}
