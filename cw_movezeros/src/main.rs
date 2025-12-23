use std::iter;
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut counter = 0;
    let mut arr_zeros = arr
        .iter()
        .filter(|x| {
            if **x != 0 {
                counter += 1;
                true
            } else {
                false
            }
        })
        .copied()
        .collect::<Vec<u8>>();
    arr_zeros.extend(vec![0; arr.len() - counter]);
    arr_zeros
}

fn move_zeros2(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .copied()
        .filter(|&x| x != 0)
        .chain(iter::repeat(0))
        .take(arr.len())
        .collect()
}

fn main() {
    println!("{:?}", move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]));
    println!("{:?}", move_zeros2(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]));

    let test_vector = [1, 2, 3, 4];
    let test_vector = test_vector
        .iter()
        .chain([5, 6, 7, 8].iter())
        .collect::<Vec<&i32>>();
    println!("{:?}", test_vector); // [1, 2, 3, 4, 5, 6, 7, 8]
}
