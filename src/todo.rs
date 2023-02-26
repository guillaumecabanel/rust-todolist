use std::fmt;

#[derive(Debug)]
pub struct Todo {
  description: String,
  done: bool,
}

impl Todo {
  pub fn new(description: String) -> Self {
      Todo {
          description,
          done: false
      }
  }
}

impl fmt::Display for Todo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let checkmark = if self.done { String::from("✅") } else { String::from("🔲")};
    write!(f, "{checkmark} {}", &self.description)
  }
}
