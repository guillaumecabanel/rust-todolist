pub use crate::todo::Todo;

#[derive(Debug)]
pub struct TodoRepository {
  records: Vec<Todo>
}

impl TodoRepository {
  pub fn new() -> Self {
    TodoRepository {
      records: Vec::new()
    }
  }

  pub fn add(&mut self, record: Todo) -> &Self {
    self.records.push(record);
    self
  }

  pub fn all(&self) -> &Vec<Todo> {
    &self.records
  }
}
