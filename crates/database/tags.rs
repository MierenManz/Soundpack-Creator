use crate::Database;
use crate::DatabaseError;

pub struct PartialTag {
  name: String,
  color: u32,
  is_favorite: bool,
}

impl PartialTag {
  pub fn new(name: String, color: u32, is_favorite: bool) -> Self {
    Self {
      name,
      color,
      is_favorite,
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn color(&self) -> u32 {
    self.color
  }

  pub fn is_favorite(&self) -> bool {
    self.is_favorite
  }
}

pub struct Tag {
  id: u32,
  name: String,
  color: u32,
  is_favorite: bool,
  creation_datetime: i64,
  edit_date: i64,
}

impl From<Tag> for PartialTag {
  fn from(value: Tag) -> Self {
    Self {
      name: value.name,
      color: value.color,
      is_favorite: value.is_favorite,
    }
  }
}

pub fn create_tag(db: &Database, tag: PartialTag) -> Result<(), DatabaseError> {
  db.execute(
    "INSERT INTO Tags (name, color_hex, is_favorite) VALUES (?1, ?2, ?3)",
    (tag.name(), format!("{}", tag.color()), tag.is_favorite()),
  )
  .expect("Error should not happen");

  Ok(())
}

pub fn read_tag_by_id(db: &Database) -> 