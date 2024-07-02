use crate::math_utilities::MatrixDouble;

pub fn add_matrices(a: &MatrixDouble, b: &MatrixDouble) -> Result<MatrixDouble, &'static str> {
    if a.nrows() != b.nrows() || a.ncols() != b.ncols() {
        return Err("Matrices must have the same dimensions for addition");
    }

    let mut result = MatrixDouble::new(a.nrows(), a.ncols());
    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            result[i][j] = a[i][j] + b[i][j];
        }
    }

    Ok(result)
}

pub fn subtract_matrices(a: &MatrixDouble, b: &MatrixDouble) -> Result<MatrixDouble, &'static str> {
    if a.nrows() != b.nrows() || a.ncols() != b.ncols() {
        return Err("Matrices must have the same dimensions for subtraction");
    }

    let mut result = MatrixDouble::new(a.nrows(), a.ncols());
    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            result[i][j] = a[i][j] - b[i][j];
        }
    }

    Ok(result)
}

pub fn multiply_matrices(a: &MatrixDouble, b: &MatrixDouble) -> Result<MatrixDouble, &'static str> {
    if a.ncols() != b.nrows() {
        return Err("Number of columns of the first matrix must equal the number of rows of the second matrix");
    }

    let mut result = MatrixDouble::new(a.nrows(), b.ncols());
    for i in 0..a.nrows() {
        for j in 0..b.ncols() {
            for k in 0..a.ncols() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Ok(result)
}
