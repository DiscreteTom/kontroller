use std::cell::RefCell;
use std::rc::Rc;

/// Return `Err` if the handle's value is 0.
pub fn check_handle(handle: u64) -> Result<u64, ()> {
  if handle == 0 {
    Err(())
  } else {
    Ok(handle)
  }
}

/// Append the data to the vector and return a reference-counted pointer to the data.
pub fn append_echo<T>(vec: &mut Vec<Rc<RefCell<T>>>, data: T) -> Rc<RefCell<T>> {
  let rc = Rc::new(RefCell::new(data));
  vec.push(rc.clone());
  rc
}
