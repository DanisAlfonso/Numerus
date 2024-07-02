// examples/lu_decomposition_example.rs

use numerus::lu_decomposition::LU;
use numerus::math_utilities::{MatrixDouble, VectorDouble};

fn main() {
    // Example matrix A (3x3 matrix)
    let a_data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0];
    let a = MatrixDouble::from_slice(3, 3, &a_data);

    // Example vector b
    let b_data = vec![2.0, -1.0, 1.0];
    let b = VectorDouble::from_slice(&b_data);

    // Create LU decomposition object
    let lu = LU::new(&a);

    // Solve the system Ax = b
    let mut x = VectorDouble::new(3);
    lu.solve(&b, &mut x);

    // Output the solution vector x
    println!("Solution vector x:");
    for i in 0..x.size() {
        println!("{:.6}", x[i]);
    }

    // Calculate the determinant
    let det = lu.det();
    println!("Determinant of A: {:.6}", det);

    // Calculate the inverse of A
    let mut a_inv = MatrixDouble::new(3, 3);
    lu.inverse(&mut a_inv);
    println!("Inverse of matrix A:");
    for i in 0..a_inv.nrows() {
        for j in 0..a_inv.ncols() {
            print!("{:.6} ", a_inv[i][j]);
        }
        println!();
    }
}
