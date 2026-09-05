use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::error::CriticalError;

#[derive(Serialize, Deserialize)]
pub struct GlobalConfig {
  db_path: PathBuf,
  locales_dir: PathBuf,
  data_dir: PathBuf,
  selected_language: Box<str>,
  // What else?
}

impl GlobalConfig {
  pub fn save(&self, path: &Path) -> Result<(), CriticalError> {
    let str = toml::to_string_pretty(self).map_err(|_| CriticalError::ConfigCouldNotWrite)?;
    fs::write(path, str).map_err(|_| CriticalError::ConfigCouldNotWrite)?;

    Ok(())
  }

  pub fn create_new(cfg_dir: &Path, data_dir: &Path) -> Self {
    Self {
      db_path: data_dir.join("database.db"),
      locales_dir: cfg_dir.join("locales"),
      data_dir: data_dir.to_path_buf(),
      selected_language: "en-us".into(),
    }
  }

  pub fn from_path(path: &Path) -> Result<Self, CriticalError> {
    let file_data = fs::read_to_string(path).map_err(|_| CriticalError::ConfigCouldNotRead)?;
    let res = toml::from_str(&file_data).map_err(|_| CriticalError::ConfigBwoken)?;

    Ok(res)
  }

  pub fn db(&self) -> &Path {
    &self.db_path
  }

  pub fn locales_dir(&self) -> &Path {
    &self.locales_dir
  }

  pub fn data_dir(&self) -> &Path {
    &self.data_dir
  }
}
