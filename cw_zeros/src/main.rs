#![allow(unused)]
//Write an algorithm that takes an array and moves all of the zeros to the end, preserving the order of the other elements.
//moveZeros([false,1,0,1,2,0,1,3,"a"]) // returns[false,1,1,2,1,3,"a",0,0]

fn main() {
    //    let results = move_zeros([false, 1, 0, 1, 2, 0, 1, 3, "a"]);
    //    for i in results {
    //        println!("{}", i);
    //    }
    let collected_iterator = (0..10).collect::<Vec<u32>>();
    println!("Collected (0..10) into: {:?}", collected_iterator);
}

//fn move_zeros(arr: &[u8]) -> Vec<u8> {
//    return [false, 1, 1, 2, 1, 3, "a", 0, 0];
//}
