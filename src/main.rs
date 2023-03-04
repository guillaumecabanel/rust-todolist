mod todo;
use todo::Todo;
mod todo_repository;
use todo_repository::TodoRepository;
mod todo_view;
mod todo_controller;
use crate::todo_controller::TodoController;
mod router;
use router::Router;

fn main() {
    let mut todo_repo = TodoRepository::new();
    let task1 = Todo::new(String::from("Learn Rust"));
    todo_repo.add(task1);
    let task2 = Todo::new(String::from("Clean room"));
    todo_repo.add(task2);
    let todo_controller = TodoController::new(todo_repo);

    let router = Router::new(todo_controller);
    router.run();
}
