mod todo;
use todo::Todo;
mod todo_repository;
use todo_repository::TodoRepository;

fn main() {
    let mut todo_repo = TodoRepository::new();
    let task1 = Todo::new(String::from("Learn Rust"));
    todo_repo.add(task1);
    let task2 = Todo::new(String::from("Clean room"));
    todo_repo.add(task2);

    for todo in todo_repo.all() {
        println!("{}", todo)
    }
}
