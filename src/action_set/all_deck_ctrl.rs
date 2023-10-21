use crate::utils::check_handle;
use steamworks::{ClientManager, Input};

pub struct AllDeckControls {
  /// The action set handle.
  pub handle: u64,
  // action handles
  pub btn_a: u64,
  pub btn_b: u64,
  pub btn_x: u64,
  pub btn_y: u64,
  pub btn_lb: u64,
  pub btn_rb: u64,
  pub btn_l4: u64,
  pub btn_r4: u64,
  pub btn_l5: u64,
  pub btn_r5: u64,
  pub btn_start: u64,
  pub btn_select: u64,
  pub lt: u64,
  pub rt: u64,
  pub move1: u64,
  pub move2: u64,
  pub move3: u64,
  pub mouse1: u64,
  pub mouse2: u64,
  pub mouse3: u64,
}

impl AllDeckControls {
  /// Return `Ok` if all handles are valid.
  pub fn new(input: &Input<ClientManager>) -> Result<Self, ()> {
    Ok(Self {
      handle: check_handle(input.get_action_set_handle("AllDeckControls"))?,
      btn_a: check_handle(input.get_digital_action_handle("btn_a"))?,
      btn_b: check_handle(input.get_digital_action_handle("btn_b"))?,
      btn_x: check_handle(input.get_digital_action_handle("btn_x"))?,
      btn_y: check_handle(input.get_digital_action_handle("btn_y"))?,
      btn_lb: check_handle(input.get_digital_action_handle("btn_lb"))?,
      btn_rb: check_handle(input.get_digital_action_handle("btn_rb"))?,
      btn_l4: check_handle(input.get_digital_action_handle("btn_l4"))?,
      btn_r4: check_handle(input.get_digital_action_handle("btn_r4"))?,
      btn_l5: check_handle(input.get_digital_action_handle("btn_l5"))?,
      btn_r5: check_handle(input.get_digital_action_handle("btn_r5"))?,
      btn_start: check_handle(input.get_digital_action_handle("btn_start"))?,
      btn_select: check_handle(input.get_digital_action_handle("btn_select"))?,
      lt: check_handle(input.get_analog_action_handle("left_trigger"))?,
      rt: check_handle(input.get_analog_action_handle("right_trigger"))?,
      move1: check_handle(input.get_analog_action_handle("Move1"))?,
      move2: check_handle(input.get_analog_action_handle("Move2"))?,
      move3: check_handle(input.get_analog_action_handle("Move3"))?,
      mouse1: check_handle(input.get_analog_action_handle("Mouse1"))?,
      mouse2: check_handle(input.get_analog_action_handle("Mouse2"))?,
      mouse3: check_handle(input.get_analog_action_handle("Mouse3"))?,
    })
  }
}
