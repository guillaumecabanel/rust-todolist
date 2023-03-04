use std::io;
use crate::todo_controller::TodoController;

pub struct Router {
  todo_controller: TodoController
}

impl Router {
  pub fn new(todo_controller: TodoController) -> Self {
    Router {
      todo_controller
    }
  }

  pub fn run(&self) -> () {
    loop {
      println!("1. List task");
      println!("2. Add task");
      println!("0. Quit");
      println!("-----------");
      println!("Action? >");

      let mut action = String::new();

      io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");

      let action: u8 = match action.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      match action {
        1 => self.todo_controller.list(),
        2 => println!("Add task"),
        _ => {
          println!("Bye");
          break;
        }
      }
    }
  }
}
