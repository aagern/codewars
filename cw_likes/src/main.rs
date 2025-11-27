// []                                -->  "no one likes this"
// ["Peter"]                         -->  "Peter likes this"
// ["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
// ["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
// ["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"

// PYTHON SOULTION:
//-------------------
// def likes(names):
//     if not names:
//         return 'no one likes this'
//     elif len(names) == 1: return '{} likes this'.format(names[0])
//     elif len(names) == 2: return '{} and {} like this'.format(names[0], names[1])
//     elif len(names) == 3: return '{}, {} and {} like this'.format(names[0], names[1], names[2])
//     else: return '{}, {} and {} others like this'.format(names[0], names[1], len(names)-2)

fn main() {
    let result = likes2(&["Alex", "Jacob", "Mark", "Max"]);
    println!("Fn result: {}", result);
}

#[allow(dead_code)]
fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        x => format!("{}, {} and {} others like this", names[0], names[1], x - 2),
    }
}

fn likes2(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".to_string(),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, other @ ..] => format!("{}, {} and {} others like this", a, b, other.len()),
    }
}
