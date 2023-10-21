use super::model::Action;
use crate::utils::{append_echo, check_handle};
use std::cell::RefCell;
use std::rc::Rc;
use steamworks::{ClientManager, Input};
use steamworks_sys::InputHandle_t;

pub struct AllDeckControls {
  /// The action set handle.
  pub handle: InputHandle_t,
  // actions
  pub actions: Vec<Rc<RefCell<Action>>>,
  pub btn_a: Rc<RefCell<Action>>,
  pub btn_b: Rc<RefCell<Action>>,
  pub btn_x: Rc<RefCell<Action>>,
  pub btn_y: Rc<RefCell<Action>>,
  pub btn_lb: Rc<RefCell<Action>>,
  pub btn_rb: Rc<RefCell<Action>>,
  pub btn_l4: Rc<RefCell<Action>>,
  pub btn_r4: Rc<RefCell<Action>>,
  pub btn_l5: Rc<RefCell<Action>>,
  pub btn_r5: Rc<RefCell<Action>>,
  pub btn_start: Rc<RefCell<Action>>,
  pub btn_select: Rc<RefCell<Action>>,
  pub lt: Rc<RefCell<Action>>,
  pub rt: Rc<RefCell<Action>>,
  pub move1: Rc<RefCell<Action>>,
  pub move2: Rc<RefCell<Action>>,
  pub move3: Rc<RefCell<Action>>,
  pub mouse1: Rc<RefCell<Action>>,
  pub mouse2: Rc<RefCell<Action>>,
  pub mouse3: Rc<RefCell<Action>>,
}

impl AllDeckControls {
  /// Return `Ok` if all handles are valid.
  pub fn new(input: &Input<ClientManager>) -> Result<Self, ()> {
    let mut actions = Vec::new();
    Ok(Self {
      handle: check_handle(input.get_action_set_handle("AllDeckControls"))?,
      btn_a: append_echo(&mut actions, Action::digital(input, "btn_a")?),
      btn_b: append_echo(&mut actions, Action::digital(input, "btn_b")?),
      btn_x: append_echo(&mut actions, Action::digital(input, "btn_x")?),
      btn_y: append_echo(&mut actions, Action::digital(input, "btn_y")?),
      btn_lb: append_echo(&mut actions, Action::digital(input, "btn_lb")?),
      btn_rb: append_echo(&mut actions, Action::digital(input, "btn_rb")?),
      btn_l4: append_echo(&mut actions, Action::digital(input, "btn_l4")?),
      btn_r4: append_echo(&mut actions, Action::digital(input, "btn_r4")?),
      btn_l5: append_echo(&mut actions, Action::digital(input, "btn_l5")?),
      btn_r5: append_echo(&mut actions, Action::digital(input, "btn_r5")?),
      btn_start: append_echo(&mut actions, Action::digital(input, "btn_start")?),
      btn_select: append_echo(&mut actions, Action::digital(input, "btn_select")?),
      lt: append_echo(&mut actions, Action::analog(input, "left_trigger")?),
      rt: append_echo(&mut actions, Action::analog(input, "right_trigger")?),
      move1: append_echo(&mut actions, Action::analog(input, "Move1")?),
      move2: append_echo(&mut actions, Action::analog(input, "Move2")?),
      move3: append_echo(&mut actions, Action::analog(input, "Move3")?),
      mouse1: append_echo(&mut actions, Action::analog(input, "Mouse1")?),
      mouse2: append_echo(&mut actions, Action::analog(input, "Mouse2")?),
      mouse3: append_echo(&mut actions, Action::analog(input, "Mouse3")?),
      actions,
    })
  }

  pub fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t) {
    for action in &mut self.actions {
      action.borrow_mut().update(input, input_handle);
    }
  }
}
