use crate::config::GlobalConfig;
use crate::error::CriticalError;
use freya::prelude::*;
use freya::router::*;
use spc_ui::Route;

use dirs;
use freya::prelude::LaunchConfig;
use freya::prelude::WindowConfig;
use freya::prelude::launch;
use std::fs;
use std::path::Path;

const APP_NAME: &'static str = env!("CARGO_BIN_NAME");

fn try_create_dir(path: &Path) -> Result<(), CriticalError> {
  fs::create_dir_all(path).map_err(|_| {
    CriticalError::DirCouldNotBeCreated(path.to_str().unwrap_or("").to_string())
  })
}

pub fn setup() -> Result<GlobalConfig, CriticalError> {
  let cfg_dir = match dirs::config_dir() {
    Some(v) => Ok(v.join(APP_NAME)),
    None => Err(CriticalError::MissingDir),
  }?;

  if !cfg_dir.exists() {
    try_create_dir(&cfg_dir)?;
  }

  let locales_dir = cfg_dir.join("locales");

  if !locales_dir.exists() {
    try_create_dir(&locales_dir)?;
  }

  let data_dir = match dirs::data_dir() {
    Some(v) => Ok(v.join(APP_NAME)),
    None => Err(CriticalError::MissingDir),
  }?;

  if !data_dir.exists() {
    try_create_dir(&data_dir)?;
  }

  let sfx_dir = data_dir.join("sfx");

  if !sfx_dir.exists() {
    try_create_dir(&sfx_dir)?;
  }

  let config_path = cfg_dir.join("config.toml");

  let config = match config_path.exists() {
    true => GlobalConfig::from_path(&config_path)?,
    false => {
      let cfg = GlobalConfig::create_new();
      cfg.save(&config_path)?;
      cfg
    }
  };

  Ok(config)
}

pub fn launch_spc(_config: GlobalConfig) {
  let window_cfg = WindowConfig::new_app(SoundpackCreator)
    .with_min_size(1280., 720.)
    .with_app_id(APP_NAME)
    .with_title(APP_NAME);

  let launch_cfg = LaunchConfig::new()
    // .with_global_context(value)
    .with_window(window_cfg);

  launch(launch_cfg);
}

struct SoundpackCreator;

impl App for SoundpackCreator {
  fn render(&self) -> impl IntoElement {
    use_init_theme(dark_theme);
    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::MainPage))
  }
}
