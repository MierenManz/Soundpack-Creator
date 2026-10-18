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

fn try_create_dir(path: &Path) -> Result<(), CriticalError> {
  fs::create_dir_all(path).map_err(|_| {
    CriticalError::DirCouldNotBeCreated(path.to_str().unwrap_or("").to_string())
  })
}

fn main() {
  let cfg_dir = some_or_critical(dirs::config_dir(), "Could not find config directory")
    .join(APP_NAME);

  if !cfg_dir.exists() {
    ok_or_critical(try_create_dir(&cfg_dir));
  }

  let locales_dir = cfg_dir.join("locales");

  if !locales_dir.exists() {
    ok_or_critical(try_create_dir(&locales_dir));
  }

  let data_dir =
    some_or_critical(dirs::data_dir(), "Could not find data directory").join(APP_NAME);

  if !data_dir.exists() {
    ok_or_critical(try_create_dir(&data_dir));
  }

  let sfx_dir = data_dir.join("sfx");

  if !sfx_dir.exists() {
    ok_or_critical(try_create_dir(&sfx_dir));
  }

  let config_path = cfg_dir.join("config.toml");

  let config = match config_path.exists() {
    true => ok_or_critical(GlobalConfig::from_path(&config_path)),
    false => {
      let cfg = GlobalConfig::create_new();
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
