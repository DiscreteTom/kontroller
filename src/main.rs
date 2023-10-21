use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use std::{thread, time::Duration};
use steamworks::Client;

pub fn main() -> iced::Result {
  thread::spawn(|| {
    // TODO: replace 480 with the real AppID
    let (client, single) = Client::init_app(480).unwrap();

    let input = client.input();
    input.init(false);

    let (fps_controls_action_set, attack_action_handle, attack2_action_handle) = loop {
      single.run_callbacks();
      let fps_controls_action_set = input.get_action_set_handle("FPSControls");
      let attack_action_handle = input.get_digital_action_handle("attack");
      let attack2_action_handle = input.get_digital_action_handle("attack2");

      println!("fps_controls_action_set: {:?}", fps_controls_action_set);
      println!("attack_action_handle: {:?}", attack_action_handle);
      println!("attack2_action_handle: {:?}", attack2_action_handle);

      if fps_controls_action_set != 0 && attack_action_handle != 0 && attack2_action_handle != 0 {
        break (
          fps_controls_action_set,
          attack_action_handle,
          attack2_action_handle,
        );
      }
      // TODO: replace 1000 with the real frame rate
      thread::sleep(Duration::from_millis(1000));
    };

    let input_handles = loop {
      single.run_callbacks();
      let handles = input.get_connected_controllers();
      println!("input_handles: {:?}", handles);
      if !handles.is_empty() {
        break handles;
      }
      // TODO: replace 1000 with the real frame rate
      thread::sleep(Duration::from_millis(1000));
    };

    input.activate_action_set_handle(input_handles[0], fps_controls_action_set);

    loop {
      single.run_callbacks();

      let value = input.get_digital_action_data(input_handles[0], attack_action_handle);
      println!("attack: {}", value.bState);

      // TODO: replace 1000 with the real frame rate
      thread::sleep(Duration::from_millis(1000));
    }
  });

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
