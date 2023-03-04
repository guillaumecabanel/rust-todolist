pub use crate::todo::Todo;

pub struct TodoView {

}

impl TodoView {
  pub fn new() -> Self {
    TodoView {

    }
  }

  pub fn list_todos(&self, todos: &Vec<Todo>) -> () {
    for todo in todos {
        println!("{}", todo)
    }
  }
}
