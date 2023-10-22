mod action_set;
mod input_thread;
mod ui_thread;
mod utils;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> iced::Result {
  let update_lock = Arc::new(Mutex::new(false));
  let (tx, rx) = mpsc::channel::<String>();

  // TODO: replace 480 with the real AppID
  // TODO: replace 1000 with the real interval
  input_thread::spawn(480, 10, update_lock.clone(), tx).unwrap();

  ui_thread::run(30, update_lock, rx)
}
