/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Returns the 'r'th row of a Matrix 'mat'
fn get_row(mat: &Matrix, r: usize) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::new();
    for col in mat {
        vec.push(col[r]);
    }
    vec
}

/// Computes the product of the vectors 'row' and 'col'.
fn vec_mult(row: &Vec<f32>, col: &Vec<f32>) -> f32 {
    let mut result: f32 = 0.;
    for i in 0..row.len() {
        result += row[i] * col[i];
    }
    result
}

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // Ensure dimensions are compatible
    assert_eq!(mat1.len(), mat2[0].len());
    // Compute desired dimensions of resulting matrix
    let rows: usize = mat1[0].len();
    let cols: usize = mat2.len();
    // Create empty matrix (0.0 for every index) of desired dimensions
    let mut result: Matrix = vec![vec![0.; rows]; cols];
    // Compute product
    let mut row: Vec<f32>;
    let mut col: &Vec<f32>;
    for i in 0..mat1[0].len() { //for each row of mat1
        row = get_row(mat1, i);
        for j in 0..mat2.len() { //for each column of mat2
            col = &mat2[j];
            result[j][i] = vec_mult(&row, col);
        }
    }
    result
}
