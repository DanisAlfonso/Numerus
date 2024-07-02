use numerus::lu_decomposition::LU;
use numerus::math_utilities::{MatrixDouble, VectorDouble};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

fn parse_matrix(input: &str) -> Result<MatrixDouble, &'static str> {
    let rows: Vec<&str> = input.split(';').map(|s| s.trim()).collect();
    let mut data = Vec::new();
    let mut ncols = None;

    for row in &rows {
        let cols: Vec<&str> = row.split_whitespace().collect();
        if ncols.is_none() {
            ncols = Some(cols.len());
        } else if ncols != Some(cols.len()) {
            return Err("All rows must have the same number of columns");
        }

        for col in cols {
            match col.parse::<f64>() {
                Ok(num) => data.push(num),
                Err(_) => return Err("Invalid number in matrix"),
            }
        }
    }

    match ncols {
        Some(cols) => Ok(MatrixDouble::from_slice(rows.len(), cols, &data)),
        None => Err("Invalid matrix format"),
    }
}

fn parse_vector(input: &str) -> Result<VectorDouble, &'static str> {
    let cols: Vec<&str> = input.split_whitespace().collect();
    let mut data = Vec::new();

    for col in cols {
        match col.parse::<f64>() {
            Ok(num) => data.push(num),
            Err(_) => return Err("Invalid number in vector"),
        }
    }

    Ok(VectorDouble::from_slice(&data))
}

fn print_matrix(matrix: &MatrixDouble) {
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            print!("{:.6} ", matrix[i][j]);
        }
        println!();
    }
}

pub fn start_repl() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut matrices: HashMap<String, MatrixDouble> = HashMap::new();

    println!("Numerus REPL");
    println!("Type 'help' for a list of commands.");
    println!("Type 'exit' to quit.");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let trimmed_line = line.trim();
                if trimmed_line == "exit" {
                    break;
                } else if trimmed_line == "help" {
                    println!("Available commands:");
                    println!("  A = [1 2 3; 4 5 6; 7 8 9] - Define a matrix");
                    println!("  inv(A) - Compute the inverse of matrix A");
                    println!("  A - Display the matrix A");
                    println!("  help - Show this help message");
                    println!("  exit - Exit the REPL");
                } else if let Some((var, matrix_str)) = trimmed_line.split_once('=') {
                    let var = var.trim();
                    let matrix_str = matrix_str.trim();
                    if matrix_str.starts_with('[') && matrix_str.ends_with(']') {
                        match parse_matrix(&matrix_str[1..matrix_str.len() - 1]) {
                            Ok(matrix) => {
                                matrices.insert(var.to_string(), matrix);
                                println!("Matrix {} defined.", var);
                            }
                            Err(err) => println!("Error: {}", err),
                        }
                    } else {
                        println!("Invalid matrix format.");
                    }
                } else if trimmed_line.starts_with("inv(") && trimmed_line.ends_with(")") {
                    let var = &trimmed_line[4..trimmed_line.len() - 1];
                    if let Some(matrix) = matrices.get(var) {
                        if matrix.nrows() == matrix.ncols() {
                            let lu = LU::new(matrix);
                            let mut inv_matrix = MatrixDouble::new(matrix.nrows(), matrix.ncols());
                            lu.inverse(&mut inv_matrix);
                            println!("Inverse of matrix {}:", var);
                            print_matrix(&inv_matrix);
                        } else {
                            println!("Matrix {} is not square and cannot be inverted.", var);
                        }
                    } else {
                        println!("Matrix {} is not defined.", var);
                    }
                } else if matrices.contains_key(trimmed_line) {
                    if let Some(matrix) = matrices.get(trimmed_line) {
                        println!("Matrix {}:", trimmed_line);
                        print_matrix(matrix);
                    }
                } else {
                    println!("Unknown command: {}", trimmed_line);
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt").unwrap();
}

