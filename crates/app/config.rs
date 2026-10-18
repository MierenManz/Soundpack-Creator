use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::error::CriticalError;

#[derive(Serialize, Deserialize)]
pub struct GlobalConfig {
  selected_language: Box<str>,
  // What else?
}

impl GlobalConfig {
  pub fn save(&self, path: &Path) -> Result<(), CriticalError> {
    let str =
      toml::to_string_pretty(self).map_err(|_| CriticalError::ConfigCouldNotWrite)?;
    fs::write(path, str).map_err(|_| CriticalError::ConfigCouldNotWrite)?;

    Ok(())
  }

  pub fn create_new() -> Self {
    Self {
      selected_language: "en-us".into(),
    }
  }

  pub fn from_path(path: &Path) -> Result<Self, CriticalError> {
    let file_data =
      fs::read_to_string(path).map_err(|_| CriticalError::ConfigCouldNotRead)?;
    let res = toml::from_str(&file_data).map_err(|_| CriticalError::ConfigBwoken)?;

    Ok(res)
  }
}
