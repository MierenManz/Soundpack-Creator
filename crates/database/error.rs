use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum DatabaseError {
  DatabaseIsNewerThanCurrentVersion,
  DatabaseIsBwoken,
  DatabaseMissing,
  CannotInitializeDB,
}

impl DatabaseError {
  fn pretty(&self) -> &'static str {
    match self {
      Self::CannotInitializeDB => "Database could not be initialized",
      Self::DatabaseIsNewerThanCurrentVersion => {
        "Database schema is newer than current version supports"
      }
      Self::DatabaseIsBwoken => "uh oh database bwoken >.<",
      Self::DatabaseMissing => "Oopsie, Scarlett deleted the database!!!",
    }
  }
}

impl Display for DatabaseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.pretty())
  }
}

impl Error for DatabaseError {}
