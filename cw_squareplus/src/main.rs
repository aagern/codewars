#[allow(unused)]
fn generate_shape(n: i32) -> String {
    let mut accum_sq: i32 = 0;
    let mut accum: i32 = 0;
    let mut result: Vec<String> = Vec::new();
    while accum_sq < n {
        while accum < n {
            accum += 1;
            result.push("+".to_string());
        }
        accum_sq += 1;
        result.push("\n".to_string());
        accum = 0;
    }
    let fin_result = &result[0..&result.len() - 1];
    fin_result.join("")
}

#[allow(unused)]
fn generate_shape2(n: i32) -> String {
    vec!["+".repeat(n as usize); n as usize].join("\n")
}

#[allow(unused)]
fn generate_shape3(n: i32) -> String {
    std::iter::repeat_n(
        std::iter::repeat_n("+", n as usize).collect::<String>(),
        n as usize,
    )
    .collect::<Vec<_>>()
    .join("\n")
}

fn main() {
    println!("{}", generate_shape3(4));
}
