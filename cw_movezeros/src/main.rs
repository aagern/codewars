fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let arr_zeros = arr.to_vec();
    println!("{:?}", arr_zeros);
    arr_zeros
}

fn main() {
    println!("{:?}", move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]));
}
