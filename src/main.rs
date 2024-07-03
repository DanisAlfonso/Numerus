mod gui;
mod lu_decomposition;
mod math_utilities;
mod matrix_operations;

use druid::AppLauncher;
use gui::{main_window, AppState};
use std::collections::HashMap;

fn main() {
    let initial_state = AppState {
        current_input: String::new(),
        matrices: HashMap::new(),
        output: String::new(),
    };

    AppLauncher::with_window(main_window())
        .launch(initial_state)
        .expect("Failed to launch application");
}

