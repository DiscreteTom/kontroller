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

    let (
      all_deck_ctrl_set,
      btn_a,
      btn_b,
      btn_x,
      btn_y,
      btn_lb,
      btn_rb,
      btn_l4,
      btn_r4,
      btn_l5,
      btn_r5,
      btn_start,
      btn_select,
      lt,
      rt,
      move1,
      move2,
      move3,
      mouse1,
      mouse2,
      mouse3,
    ) = pool(&single, 100, || {
      let all_deck_ctrl_set = input.get_action_set_handle("AllDeckControls");
      let btn_a = input.get_digital_action_handle("btn_a");
      let btn_b = input.get_digital_action_handle("btn_b");
      let btn_x = input.get_digital_action_handle("btn_x");
      let btn_y = input.get_digital_action_handle("btn_y");
      let btn_lb = input.get_digital_action_handle("btn_lb");
      let btn_rb = input.get_digital_action_handle("btn_rb");
      let btn_l4 = input.get_digital_action_handle("btn_l4");
      let btn_r4 = input.get_digital_action_handle("btn_r4");
      let btn_l5 = input.get_digital_action_handle("btn_l5");
      let btn_r5 = input.get_digital_action_handle("btn_r5");
      let btn_start = input.get_digital_action_handle("btn_start");
      let btn_select = input.get_digital_action_handle("btn_select");
      let lt = input.get_analog_action_handle("left_trigger");
      let rt = input.get_analog_action_handle("right_trigger");
      let move1 = input.get_analog_action_handle("Move1");
      let move2 = input.get_analog_action_handle("Move2");
      let move3 = input.get_analog_action_handle("Move3");
      let mouse1 = input.get_analog_action_handle("Mouse1");
      let mouse2 = input.get_analog_action_handle("Mouse2");
      let mouse3 = input.get_analog_action_handle("Mouse3");

      if all_deck_ctrl_set != 0
        && btn_a != 0
        && btn_b != 0
        && btn_x != 0
        && btn_y != 0
        && btn_lb != 0
        && btn_rb != 0
        && btn_l4 != 0
        && btn_r4 != 0
        && btn_l5 != 0
        && btn_r5 != 0
        && btn_start != 0
        && btn_select != 0
        && lt != 0
        && rt != 0
        && move1 != 0
        && move2 != 0
        && move3 != 0
        && mouse1 != 0
        && mouse2 != 0
        && mouse3 != 0
      {
        println!("action set ready");
        return Some((
          all_deck_ctrl_set,
          btn_a,
          btn_b,
          btn_x,
          btn_y,
          btn_lb,
          btn_rb,
          btn_l4,
          btn_r4,
          btn_l5,
          btn_r5,
          btn_start,
          btn_select,
          lt,
          rt,
          move1,
          move2,
          move3,
          mouse1,
          mouse2,
          mouse3,
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

    input.activate_action_set_handle(input_handles[0], all_deck_ctrl_set);

    // TODO: replace 1000 with the real interval
    pool(&single, 1000, || {
      let value = input.get_digital_action_data(input_handles[0], btn_a);
      println!("btn_a: {}", value.bState);
      Option::<()>::None // run forever
    });
  });

  Ok(())
}
