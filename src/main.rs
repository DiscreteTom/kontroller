use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
use std::{thread, time::Duration};
use steamworks::{Client, SingleClient};

fn pool<T, F>(single: &SingleClient, interval_ms: u64, f: F) -> T
where
  F: Fn() -> Option<T>,
{
  loop {
    single.run_callbacks();

    match f() {
      Some(v) => {
        return v;
      }
      None => {}
    }

    thread::sleep(Duration::from_millis(interval_ms));
  }
}

pub fn main() -> iced::Result {
  thread::spawn(|| {
    // TODO: replace 480 with the real AppID
    let (client, single) = Client::init_app(480).unwrap();

    let input = client.input();
    input.init(false);

    let (fps_controls_action_set, attack_action_handle, attack2_action_handle) =
      pool(&single, 100, || {
        let fps_controls_action_set = input.get_action_set_handle("FPSControls");
        let attack_action_handle = input.get_digital_action_handle("attack");
        let attack2_action_handle = input.get_digital_action_handle("attack2");

        if fps_controls_action_set != 0 && attack_action_handle != 0 && attack2_action_handle != 0 {
          println!("action set ready");
          return Some((
            fps_controls_action_set,
            attack_action_handle,
            attack2_action_handle,
          ));
        }
        return None;
      });

    let input_handles = pool(&single, 300, || {
      let handles = input.get_connected_controllers();
      if !handles.is_empty() {
        println!("num of input handles: {:?}", handles.len());
        return Some(handles);
      }
      return None;
    });

    input.activate_action_set_handle(input_handles[0], fps_controls_action_set);

    // TODO: replace 1000 with the real interval
    pool(&single, 1000, || {
      let value = input.get_digital_action_data(input_handles[0], attack_action_handle);
      println!("attack: {}", value.bState);
      Option::<()>::None // run forever
    });
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
