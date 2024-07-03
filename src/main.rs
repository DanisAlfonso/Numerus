use druid::{AppLauncher, WindowDesc};

mod gui;
mod lu_decomposition;
mod math_utilities;
mod matrix_operations;

fn main() {
    let main_window = gui::main_window();
    let initial_state = gui::AppState {
        current_input: String::new(),
        matrices: std::collections::HashMap::new(),
        history: druid::im::Vector::new(),
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}

