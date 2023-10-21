use crate::utils::check_handle;
use steamworks::{ClientManager, Input};
use steamworks_sys::{
  InputAnalogActionData_t, InputDigitalActionData_t, InputDigitalActionHandle_t, InputHandle_t,
};

#[derive(Clone, Debug)]
pub enum ActionData {
  Analog(Option<InputAnalogActionData_t>),
  Digital(Option<InputDigitalActionData_t>),
}

// TODO: add `enabled`
#[derive(Clone, Debug)]
pub struct Action {
  pub handle: InputDigitalActionHandle_t,
  pub data: ActionData,
}

impl Action {
  /// Create a new analog action.
  /// Return `Ok` if the handle is valid.
  pub fn analog(input: &Input<ClientManager>, action_name: &str) -> Result<Self, ()> {
    Ok(Self {
      handle: check_handle(input.get_analog_action_handle(action_name))?,
      data: ActionData::Analog(None),
    })
  }

  /// Create a new digital action.
  /// Return `Ok` if the handle is valid.
  pub fn digital(input: &Input<ClientManager>, action_name: &str) -> Result<Self, ()> {
    Ok(Self {
      handle: check_handle(input.get_digital_action_handle(action_name))?,
      data: ActionData::Digital(None),
    })
  }

  /// Update the action data.
  pub fn update(&mut self, input: &Input<ClientManager>, input_handle: InputHandle_t) {
    match &mut self.data {
      ActionData::Analog(data) => {
        *data = Some(input.get_analog_action_data(input_handle, self.handle));
      }
      ActionData::Digital(data) => {
        *data = Some(input.get_digital_action_data(input_handle, self.handle));
      }
    }
  }
}
