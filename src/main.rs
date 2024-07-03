use gui::NumerusApp;
use iced::{Application, Settings};

mod gui;

fn main() {
    NumerusApp::run(Settings::default()).unwrap();
}

