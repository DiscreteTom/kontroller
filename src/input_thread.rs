use crate::action_set::{all_deck_ctrl::AllDeckControls, model::ActionData};
use std::{thread, time::Duration};
use steamworks::{Client, SResult, SingleClient};

/// Run a function until it returns a value.
/// If the function returns `None`, wait for the specified interval and run the Steam callbacks.
fn pool<R, F>(single: &SingleClient, interval_ms: u64, mut f: F) -> R
where
  F: FnMut() -> Option<R>,
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

pub fn spawn(app_id: u32, interval: u64) -> SResult<()> {
  let (client, single) = Client::init_app(app_id)?;

  thread::spawn(move || {
    let input = client.input();
    input.init(false);

    let mut all_deck_ctrl = pool(&single, 100, || match AllDeckControls::new(&input) {
      Ok(c) => Some(c),
      Err(_) => None,
    });

    let input_handles = pool(&single, 100, || {
      let handles = input.get_connected_controllers();
      if !handles.is_empty() {
        println!("num of input handles: {:?}", handles.len());
        return Some(handles);
      }
      return None;
    });

    input.activate_action_set_handle(input_handles[0], all_deck_ctrl.handle);

    pool(&single, interval, || {
      all_deck_ctrl.update(&input, input_handles[0]);

      // for debug
      match all_deck_ctrl.btn_a.borrow().data {
        ActionData::Digital(Some(data)) => {
          if data.bState {
            println!("btn_a pressed");
          }
        }
        _ => {}
      }

      Option::<()>::None // run forever
    });
  });

  Ok(())
}
