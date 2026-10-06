mod app;
mod config;
mod error;

use crate::config::GlobalConfig;
use crate::error::CriticalError;
use app::SoundpackCreator;
use dirs;
use freya::prelude::LaunchConfig;
use freya::prelude::WindowConfig;
use freya::prelude::launch;
use spc_utils::report::ok_or_critical;
use spc_utils::report::some_or_critical;
use std::fs;
use std::path::Path;

const APP_NAME: &'static str = env!("CARGO_BIN_NAME");

fn setup_data_directory(base_dir: &Path) -> Result<(), CriticalError> {
  fs::create_dir_all(base_dir.join("sfx")).map_err(|_| CriticalError::DataDirCannotBeCreated)
}

fn setup_cfg_directory(base_dir: &Path) -> Result<(), CriticalError> {
  fs::create_dir_all(base_dir.join("locales")).map_err(|_| CriticalError::ConfigDirCannotBeCreated)
}

fn main() {
  let cfg_dir =
    some_or_critical(dirs::config_dir(), "Could not find config directory").join(APP_NAME);

  let data_dir = some_or_critical(dirs::data_dir(), "Could not find data directory").join(APP_NAME);

  if !cfg_dir.exists() {
    ok_or_critical(setup_cfg_directory(&cfg_dir));
  }

  if !data_dir.exists() {
    ok_or_critical(setup_data_directory(&data_dir));
  }

  let config_path = cfg_dir.join("config.toml");

  let config = match config_path.exists() {
    true => ok_or_critical(GlobalConfig::from_path(&config_path)),
    false => {
      let cfg = GlobalConfig::create_new(&cfg_dir, &data_dir);
      ok_or_critical(cfg.save(&config_path));
      cfg
    }
  };

  let app = ok_or_critical(SoundpackCreator::new(config));

  let window_cfg = WindowConfig::new_app(app)
    .with_min_size(1280., 720.)
    .with_app_id(APP_NAME)
    .with_title(APP_NAME);

  let launch_cfg = LaunchConfig::new().with_window(window_cfg);

  launch(launch_cfg);
}
