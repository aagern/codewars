fn main() {
    let a = (0..6).map(|x| x * 2);
    for i in a {
        println!("map i = {}", i);
    }

    let b = (0..6).filter(|&x| x % 2 == 0);
    for i in b {
        println!("filter i = {}", i);
    }
}
