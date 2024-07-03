use iced::widget::Text;
use iced::{Application, Command, Element};

pub struct NumerusApp;

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for NumerusApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Numerus")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Text::new("Hello, Numerus!").into()
    }

    type Theme = iced::Theme;
}

