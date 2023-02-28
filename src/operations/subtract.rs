pub fn subtract_matrix(a: &[f32], b: &[f32], rows: usize, cols: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(rows * cols);
    for i in 0..rows {
        for j in 0..cols {
            result.push(a[i * cols + j] - b[i * cols + j]);
        }
    }
    result
}