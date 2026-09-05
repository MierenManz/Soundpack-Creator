use crate::GlobalConfig;
use crate::error::CriticalError;
use freya::prelude::*;
use freya::router::*;
use spc_database::Database;
use spc_ui::Route;

pub struct SoundpackCreator {
  db: Database,
}

impl SoundpackCreator {
  pub fn new(config: GlobalConfig) -> Result<Self, CriticalError> {
    Ok(Self {
      db: Database::open(config.db())?,
    })
  }
}

impl App for SoundpackCreator {
  fn render(&self) -> impl IntoElement {
    use_init_theme(dark_theme);
    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::MainPage))
  }
}
