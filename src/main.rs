mod action_set;
mod input_thread;
mod utils;

use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
  // TODO: replace 480 with the real AppID
  input_thread::spawn(480).unwrap();

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
    // TODO: update title by action set
    String::from("Kontroller - All Deck Controls")
  }

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Self::Message> {
    "Hello, world!".into()
  }
}
