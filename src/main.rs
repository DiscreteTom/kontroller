mod input_thread;

use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
  input_thread::spawn();

  Hello::run(Settings::default())
}

struct Hello {}

impl Application for Hello {
  type Executor = executor::Default;
  type Flags = ();
  type Message = ();
  type Theme = Theme;

  fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
    (Hello {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("A cool application")
  }

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Self::Message> {
    "Hello, world!".into()
  }
}
