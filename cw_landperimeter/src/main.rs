// pub fn land_perimeter(arr: &[&str]) -> String {
//     todo!("Your code here!")
// }

use std::collections::HashSet;

fn main() {
    let land = [
        "OXOOOX", "OXOXOO", "XXOOOX", "OXXXOO", "OOXOOX", "OXOOOO", "OOXOOX", "OOXOOO", "OXOOOO",
        "OXOOXX",
    ];

    // Collect X coordinates
    let x_coords: Vec<(usize, usize)> = land
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                //               .filter_map(move |(c, ch)| if ch == 'X' { Some((r, c)) } else { None })
                .filter_map(move |(c, ch)| (ch == 'X').then_some((r, c)))
        })
        .collect();

    let coord_set: HashSet<(usize, usize)> = x_coords.iter().cloned().collect();
    let mut adjacent_count = 0;

    for &(r, c) in &x_coords {
        // Check right neighbor
        if coord_set.contains(&(r, c + 1)) {
            adjacent_count += 1;
        }
        // Check bottom neighbor
        if coord_set.contains(&(r + 1, c)) {
            adjacent_count += 1;
        }
    }

    let result = x_coords.len() * 4 - adjacent_count * 2;

    println!("X coordinates: {:?}", x_coords);
    println!("Number os Xs: {}", x_coords.len());
    println!("Total adjacent pairs: {}", adjacent_count);
    println!("Perimeter equals: {}", result)
}
