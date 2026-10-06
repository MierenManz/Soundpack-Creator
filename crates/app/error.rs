use std::error::Error;
use std::fmt::Display;

use spc_database::DatabaseError;

#[derive(Debug)]
pub enum CriticalError {
  ConfigMissing,
  ConfigBwoken,
  CannotInitializeDB,
  DatabaseIsNewerThanCurrentVersion,
  DatabaseIsBwoken,
  DatabaseMissing,
  DataDirCannotBeCreated,
  ConfigDirCannotBeCreated,
  ConfigCouldNotWrite,
  ConfigCouldNotRead,
}

impl CriticalError {
  fn pretty(&self) -> &'static str {
    match self {
      Self::ConfigMissing => "Missing Config detected",
      Self::ConfigBwoken => "Config is broken and cannot be repaired",
      Self::CannotInitializeDB => "Database could not be initialized",
      Self::DatabaseIsNewerThanCurrentVersion => {
        "Database schema is newer than current version supports"
      }
      Self::DatabaseIsBwoken => "uh oh database bwoken >.<",
      Self::DatabaseMissing => "Oopsie, Scarlett deleted the database!!!",
      Self::DataDirCannotBeCreated => "Data directory could not be created",
      Self::ConfigDirCannotBeCreated => "Config directory could not be created",
      Self::ConfigCouldNotWrite => "Config could not be written",
      Self::ConfigCouldNotRead => "Config could not be read",
    }
  }
}

impl Display for CriticalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.pretty())
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
