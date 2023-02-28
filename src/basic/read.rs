use std::io;

pub fn read_matrix() -> Vec<f32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}