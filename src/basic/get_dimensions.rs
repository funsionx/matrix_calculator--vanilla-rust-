pub fn get_dimensions(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}
