mod gui;
mod lu_decomposition;
mod math_utilities;
mod matrix_operations;
mod repl;

use clap::{Arg, Command};
use eframe::egui;
use gui::NumerusApp;

fn main() {
    let matches = Command::new("Numerus")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Matrix operations in Rust")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Sets the mode to run in (repl or gui)")
                .default_value("repl"),
        )
        .get_matches();

    match matches
        .get_one::<String>("mode")
        .map(|s| s.as_str())
        .unwrap()
    {
        "gui" => {
            let options = eframe::NativeOptions::default();
            eframe::run_native(
                "Numerus",
                options,
                Box::new(|_cc| Box::new(NumerusApp::default())),
            );
        }
        "repl" => {
            repl::start_repl();
        }
        _ => {
            println!("Unknown mode. Use 'repl' or 'gui'.");
        }
    }
}

