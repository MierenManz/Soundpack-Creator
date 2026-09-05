use rusqlite::Connection;
use rusqlite::Result;
use std::cmp::Ordering;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use crate::DatabaseError;

const INITIALIZE_DB: &'static str = include_str!("./db_schema_v1.sql");

#[derive(Debug)]
pub struct Database {
  db_version: u8,
  inner: Arc<Connection>,
}

impl Database {
  // This can only be incremented up. NEVER DOWN
  const CURRENT_SCHEMA_VERSION: u8 = 1;

  fn get_db_version(conn: &Connection) -> u8 {
    conn
      .pragma_query_value(None, "user_version", |x| x.get(0))
      .unwrap()
  }

  pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DatabaseError> {
    let conn = Connection::open(&path).map_err(|_| DatabaseError::CannotInitializeDB)?;

    conn
      .execute_batch(INITIALIZE_DB)
      .map_err(|_| DatabaseError::CannotInitializeDB)?;

    let version = Self::get_db_version(&conn);

    match Self::CURRENT_SCHEMA_VERSION.cmp(&version) {
      Ordering::Equal => Ok(Self {
        inner: Arc::new(conn),
        db_version: version,
      }),
      Ordering::Greater => todo!("Migration is not a thing yet"),
      Ordering::Less => Err(DatabaseError::DatabaseIsNewerThanCurrentVersion),
    }
  }
}

impl Deref for Database {
  type Target = Arc<Connection>;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
