pub fn print_matrix(matrix: &[f32], rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols {
            print!("{} ", matrix[i * cols + j]);
        }
        println!();
    }
}