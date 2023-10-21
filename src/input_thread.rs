use std::{thread, time::Duration};
use steamworks::{Client, SResult, SingleClient};

/// Run a function until it returns a value.
/// If the function returns `None`, wait for the specified interval and run the Steam callbacks.
fn pool<R, F>(single: &SingleClient, interval_ms: u64, f: F) -> R
where
  F: Fn() -> Option<R>,
{
  loop {
    // call the function immediately, in case it can return a value without waiting
    match f() {
      Some(r) => return r,
      None => {}
    }

    thread::sleep(Duration::from_millis(interval_ms));
    single.run_callbacks();
  }
}

pub fn spawn(app_id: u32) -> SResult<()> {
  let (client, single) = Client::init_app(app_id)?;

  thread::spawn(move || {
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

  Ok(())
}
