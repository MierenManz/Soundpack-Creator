use std::error::Error;
use std::fmt::Display;

use spc_database::DatabaseError;

#[derive(Debug)]
pub enum CriticalError {
  MissingDir,
  // ConfigMissing,
  ConfigBwoken,
  CannotInitializeDB,
  DatabaseIsNewerThanCurrentVersion,
  DatabaseIsBwoken,
  DatabaseMissing,
  DirCouldNotBeCreated(String),
  ConfigCouldNotWrite,
  ConfigCouldNotRead,
}

impl<'a> CriticalError {
  fn pretty(&self) -> String {
    match self {
      Self::MissingDir => "Missing Directory detected".into(),
      Self::ConfigBwoken => "Config is broken and cannot be repaired".into(),
      Self::CannotInitializeDB => "Database could not be initialized".into(),
      Self::DatabaseIsNewerThanCurrentVersion => {
        "Database schema is newer than current version supports".into()
      }
      Self::DatabaseIsBwoken => "Database is broken beyond repair".into(),
      Self::DatabaseMissing => "Database is missing".into(),
      Self::DirCouldNotBeCreated(s) => format!("Directory '{s}' could not be created"),
      Self::ConfigCouldNotWrite => "Config could not be written".into(),
      Self::ConfigCouldNotRead => "Config could not be read".into(),
    }
  }
}

impl Display for CriticalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.pretty())
  }
}

impl Error for CriticalError {}

impl From<DatabaseError> for CriticalError {
  fn from(value: DatabaseError) -> Self {
    match value {
      DatabaseError::DatabaseIsNewerThanCurrentVersion => {
        Self::DatabaseIsNewerThanCurrentVersion
      }
      DatabaseError::DatabaseIsBwoken => Self::DatabaseIsBwoken,
      DatabaseError::DatabaseMissing => Self::DatabaseMissing,
      DatabaseError::CannotInitializeDB => Self::CannotInitializeDB,
    }
  }
}
