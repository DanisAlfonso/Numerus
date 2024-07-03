use druid::theme;
use druid::widget::{Button, Container, CrossAxisAlignment, Flex, Label, Scroll, TextBox};
use druid::{
    AppLauncher, Color, Data, Env, Event, EventCtx, Lens, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use std::collections::HashMap;

use crate::lu_decomposition::LU;
use crate::math_utilities::{MatrixDouble, VectorDouble};
use crate::matrix_operations::{add_matrices, multiply_matrices, subtract_matrices};
use meval::eval_str;

#[derive(Clone, Data, Lens)]
pub struct CommandEntry {
    pub input: String,
    pub output: String,
    pub index: usize,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_input: String,
    #[data(ignore)]
    pub matrices: HashMap<String, MatrixDouble>,
    pub history: druid::im::Vector<CommandEntry>,
}

pub fn build_root_widget() -> impl Widget<AppState> {
    let input = TextBox::new()
        .with_placeholder("Enter command")
        .expand_width()
        .lens(AppState::current_input)
        .controller(SubmitController);

    let button = Button::new("Run")
        .padding(10.0)
        .on_click(|_ctx, data: &mut AppState, _env| {
            submit_command(data);
        });

    let input_row = Flex::row()
        .with_flex_child(input, 1.0)
        .with_spacer(10.0)
        .with_child(button);

    let output_list = druid::widget::List::new(|| {
        Container::new(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Label::new(|item: &CommandEntry, _env: &Env| {
                        format!("In[{}]: {}", item.index, item.input)
                    })
                    .with_text_size(14.0)
                    .padding((0.0, 5.0)),
                )
                .with_child(
                    Label::new(|item: &CommandEntry, _env: &Env| {
                        format!("Out[{}]: {}", item.index, item.output)
                    })
                    .with_text_color(Color::rgb8(0, 0, 255))
                    .with_text_size(14.0)
                    .padding((0.0, 5.0)),
                )
                .with_spacer(10.0),
        )
        .background(Color::grey(0.2))
        .border(Color::grey(0.6), 1.0)
        .rounded(10.0)
        .padding(10.0)
    })
    .lens(AppState::history)
    .padding(10.0);

    let scroll = Scroll::new(output_list).vertical().expand();

    Flex::column()
        .with_child(
            Label::new("Numerus Interactive Notebook")
                .with_text_size(30.0)
                .padding(10.0),
        )
        .with_spacer(10.0)
        .with_flex_child(scroll, 1.0)
        .with_spacer(10.0)
        .with_child(input_row)
        .padding(10.0)
        .align_vertical(UnitPoint::TOP)
        .expand()
}

pub fn main_window() -> WindowDesc<AppState> {
    WindowDesc::new(build_root_widget)
        .title("Numerus")
        .window_size((800.0, 600.0))
}

pub fn main() {
    let main_window = main_window();

    let initial_state = AppState {
        current_input: String::new(),
        matrices: HashMap::new(),
        history: druid::im::Vector::new(),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
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
            return format!("Determinant of matrix {}: {}", var, det);
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
                                .map(|i| format!("{}", x[i]))
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

fn handle_matrix_operations(
    operation: char,
    a_var: &str,
    b_var: &str,
    matrices: &HashMap<String, MatrixDouble>,
) -> String {
    if let Some(a) = matrices.get(a_var) {
        if let Some(b) = matrices.get(b_var) {
            let result = match operation {
                '+' => add_matrices(a, b),
                '-' => subtract_matrices(a, b),
                '*' => multiply_matrices(a, b),
                _ => Err("Unknown operation"),
            };

            match result {
                Ok(result) => {
                    let mut output = format!("Result of {} {} {}:\n", a_var, operation, b_var);
                    output.push_str(&matrix_to_string(&result));
                    output
                }
                Err(err) => format!("Error: {}", err),
            }
        } else {
            format!("Matrix {} is not defined.", b_var)
        }
    } else {
        format!("Matrix {} is not defined.", a_var)
    }
}

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

fn matrix_to_string(matrix: &MatrixDouble) -> String {
    let mut result = String::new();
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            result.push_str(&format!("{:<10}    ", matrix[i][j]));
        }
        result.push('\n');
    }
    result
}

fn submit_command(data: &mut AppState) {
    let output = handle_command(&data.current_input, &mut data.matrices);
    let index = data.history.len() + 1;
    data.history.push_back(CommandEntry {
        input: data.current_input.clone(),
        output,
        index,
    });
    data.current_input.clear();
}

struct SubmitController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for SubmitController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        _env: &Env,
    ) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == druid::keyboard_types::Key::Enter && key_event.mods.shift() {
                submit_command(data);
                ctx.request_update();
            }
        }
        child.event(ctx, event, data, _env);
    }
}

