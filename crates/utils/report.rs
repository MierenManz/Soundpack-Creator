use crate::log::critical;
use std::error::Error;
use std::process::exit;

pub fn ok_or_critical<T, E: Error>(res: Result<T, E>) -> T {
  match res {
    Ok(v) => v,
    Err(e) => {
      critical(&format!("{e}"));
      exit(1);
    }
  }
}

pub fn some_or_critical<T>(option: Option<T>, msg: &str) -> T {
  match option {
    Some(v) => v,
    None => {
      critical(msg);
      exit(1);
    }
  }
}
