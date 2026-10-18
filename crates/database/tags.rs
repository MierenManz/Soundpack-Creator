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

// pub fn create_tag(database: &mut Database, tag: &Tag) -> Result<(), DatabaseError> {
//   Ok(())
// }

// pub fn read_tag_by_id(
//   database: &mut Database,
//   id: u32,
// ) -> Result<Option<Tag>, DatabaseError> {
//   Ok(None)
// }

// pub fn read_tags(database: &mut Database) -> Result<Vec<Tag>, DatabaseError> {
//   Ok(Vec::new())
// }
