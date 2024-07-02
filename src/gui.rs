use eframe::egui::{self, FontId, ScrollArea, TextEdit, Ui};
use eframe::App;
use meval::eval_str;
use std::collections::HashMap;

use crate::lu_decomposition::LU;
use crate::math_utilities::{MatrixDouble, VectorDouble};
use crate::matrix_operations::{add_matrices, multiply_matrices, subtract_matrices};
use crate::repl::{handle_matrix_operations, parse_matrix};

#[derive(Clone)]
struct CommandEntry {
    input: String,
    output: String,
}

pub struct NumerusApp {
    current_input: String,
    history: Vec<CommandEntry>,
    matrices: HashMap<String, MatrixDouble>,
    font_size: f32,
}

impl Default for NumerusApp {
    fn default() -> Self {
        Self {
            current_input: String::new(),
            history: Vec::new(),
            matrices: HashMap::new(),
            font_size: 16.0,
        }
    }
}

impl App for NumerusApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut changed = false;

        if ctx.input().modifiers.command && ctx.input().key_pressed(egui::Key::PlusEquals) {
            self.font_size += 1.0;
            changed = true;
        }

        if ctx.input().modifiers.command && ctx.input().key_pressed(egui::Key::Minus) {
            self.font_size -= 1.0;
            changed = true;
        }

        if changed {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(
                egui::RichText::new("Numerus Interactive Notebook")
                    .font(FontId::proportional(self.font_size + 4.0)),
            );

            ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.history {
                    display_entry(ui, entry, self.font_size);
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("In:").font(FontId::proportional(self.font_size)));
                    let input = ui.add_sized(
                        [ui.available_width() - 50.0, 30.0],
                        TextEdit::multiline(&mut self.current_input)
                            .font(FontId::proportional(self.font_size))
                            .desired_width(f32::INFINITY)
                            .desired_rows(1),
                    );

                    if ui
                        .button(
                            egui::RichText::new("Run").font(FontId::proportional(self.font_size)),
                        )
                        .clicked()
                    {
                        self.execute_command();
                    }

                    if ui.input().key_pressed(egui::Key::Enter) && ui.input().modifiers.shift {
                        self.execute_command();
                    }
                });
            });
        });
    }
}

impl NumerusApp {
    fn execute_command(&mut self) {
        let command = self.current_input.clone();
        let response = handle_command(&command, &mut self.matrices);
        self.history.push(CommandEntry {
            input: command,
            output: response,
        });
        self.current_input.clear();
    }
}

fn display_entry(ui: &mut Ui, entry: &CommandEntry, font_size: f32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("In:").font(FontId::proportional(font_size)));
        ui.add(
            egui::Label::new(
                egui::RichText::new(&entry.input).font(FontId::proportional(font_size)),
            )
            .wrap(true),
        );
    });
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Out:").font(FontId::proportional(font_size)));
        ui.add(
            egui::Label::new(
                egui::RichText::new(&entry.output).font(FontId::proportional(font_size)),
            )
            .wrap(true),
        );
    });
    ui.separator();
}

fn handle_command(command: &str, matrices: &mut HashMap<String, MatrixDouble>) -> String {
    let trimmed_command = command.trim();

    // Try to evaluate the expression first
    match eval_str(trimmed_command) {
        Ok(result) => return format!("Result: {}", result),
        Err(_) => {}
    }

    if let Some((var, matrix_str)) = trimmed_command.split_once('=') {
        let var = var.trim();
        let matrix_str = matrix_str.trim();
        if matrix_str.starts_with('[') && matrix_str.ends_with(']') {
            match parse_matrix(&matrix_str[1..matrix_str.len() - 1]) {
                Ok(matrix) => {
                    matrices.insert(var.to_string(), matrix);
                    return format!("Matrix {} defined.", var);
                }
                Err(err) => return format!("Error: {}", err),
            }
        } else {
            return "Invalid matrix format.".to_string();
        }
    } else if let Some((a_var, b_var)) = trimmed_command.split_once('+') {
        return handle_matrix_operations('+', a_var.trim(), b_var.trim(), matrices);
    } else if let Some((a_var, b_var)) = trimmed_command.split_once('-') {
        return handle_matrix_operations('-', a_var.trim(), b_var.trim(), matrices);
    } else if let Some((a_var, b_var)) = trimmed_command.split_once('*') {
        return handle_matrix_operations('*', a_var.trim(), b_var.trim(), matrices);
    } else if trimmed_command.starts_with("inv(") && trimmed_command.ends_with(")") {
        let var = &trimmed_command[4..trimmed_command.len() - 1];
        if let Some(matrix) = matrices.get(var) {
            if matrix.nrows() == matrix.ncols() {
                let lu = LU::new(matrix);
                let mut inv_matrix = MatrixDouble::new(matrix.nrows(), matrix.ncols());
                lu.inverse(&mut inv_matrix);
                return format!(
                    "Inverse of matrix {}:\n{}",
                    var,
                    matrix_to_string(&inv_matrix)
                );
            } else {
                return format!("Matrix {} is not square and cannot be inverted.", var);
            }
        } else {
            return format!("Matrix {} is not defined.", var);
        }
    } else if trimmed_command.starts_with("det(") && trimmed_command.ends_with(")") {
        let var = &trimmed_command[4..trimmed_command.len() - 1];
        if let Some(matrix) = matrices.get(var) {
            let lu = LU::new(matrix);
            let det = lu.det();
            return format!("Determinant of matrix {}: {:.6}", var, det);
        } else {
            return format!("Matrix {} is not defined.", var);
        }
    } else if trimmed_command.starts_with("solve(") && trimmed_command.ends_with(")") {
        if let Some((a_var, b_var)) = trimmed_command[6..trimmed_command.len() - 1].split_once(',')
        {
            let a_var = a_var.trim();
            let b_var = b_var.trim();
            if let Some(matrix) = matrices.get(a_var) {
                if let Some(vector) = matrices.get(b_var) {
                    if vector.ncols() == 1 && vector.nrows() == matrix.nrows() {
                        let lu = LU::new(matrix);
                        let mut x = VectorDouble::new(vector.nrows());
                        let b = VectorDouble::from_slice(vector.data());
                        lu.solve(&b, &mut x);
                        return format!(
                            "Solution vector x:\n{}",
                            (0..x.size())
                                .map(|i| format!("{:.6}", x[i]))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                    } else {
                        return format!(
                            "Vector {} is not a valid vector or does not match matrix {} dimensions.",
                            b_var, a_var
                        );
                    }
                } else {
                    return format!("Vector {} is not defined.", b_var);
                }
            } else {
                return format!("Matrix {} is not defined.", a_var);
            }
        } else {
            return "Invalid solve command format.".to_string();
        }
    } else if trimmed_command.starts_with("lu_decomposition(") && trimmed_command.ends_with(")") {
        let var = &trimmed_command[17..trimmed_command.len() - 1];
        if let Some(matrix) = matrices.get(var) {
            let lu = LU::new(matrix);
            let mut lu_matrix = MatrixDouble::new(matrix.nrows(), matrix.ncols());
            lu.lu_decomposition(matrix, &mut lu_matrix);
            return format!(
                "LU decomposition of matrix {}:\n{}",
                var,
                matrix_to_string(&lu_matrix)
            );
        } else {
            return format!("Matrix {} is not defined.", var);
        }
    } else if matrices.contains_key(trimmed_command) {
        if let Some(matrix) = matrices.get(trimmed_command) {
            return format!("Matrix {}:\n{}", trimmed_command, matrix_to_string(matrix));
        }
    } else {
        return format!("Unknown command: {}", trimmed_command);
    }

    "Unknown command".to_string()
}

fn matrix_to_string(matrix: &MatrixDouble) -> String {
    let mut result = String::new();
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            result.push_str(&format!("{:.6} ", matrix[i][j]));
        }
        result.push('\n');
    }
    result
}

