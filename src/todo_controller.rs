pub use crate::todo_repository::TodoRepository;
pub use crate::todo_view::TodoView;

pub struct TodoController {
  todo_repository: TodoRepository,
  view: TodoView
}

impl TodoController {
  pub fn new(todo_repository: TodoRepository) -> Self {
    TodoController {
      todo_repository,
      view: TodoView::new()
    }
  }

  pub fn list(&self) -> () {
    self.view.list_todos(self.todo_repository.all())
  }
}
