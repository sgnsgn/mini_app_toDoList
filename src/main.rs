use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::io;

#[derive(Debug, Clone)]
struct Task {
    id: Uuid,
    description: String,
    created_at: DateTime<Utc>,
    is_done: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            description,
            created_at: Utc::now(),
            is_done: false,
        }
    }
}

struct ToDoList {
    task_list: Vec<Task>,
  }
  
  impl ToDoList {
  fn new() -> Self {
    ToDoList {
      task_list: Vec::new(),
    }
  }
    // Add a new task to the list
  fn add_task(&mut self, description: String) {
    let task = Task::new(description);
    self.task_list.push(task);
  }

  fn display_tasks(&self) {
    for (i, task) in self.task_list.iter().enumerate() {
        let status = if task.is_done { "✔" } else { "✗" };
        println!("{}. [{}] {}", i + 1, status, task.description);
    }
}

fn mark_done(&mut self, index: usize) {
    match self.task_list.get_mut(index) {
        Some(task) => task.is_done = true,
        None => println!("Invalid task index: {}", index),
    }
}

fn delete_task(&mut self, index: usize) {
  if index == 0 || index > self.task_list.len() {
      println!("Invalid task index: {}", index);
      return;
  }
  println!("Task to delete: {}", self.task_list[index - 1].description);
  let mut input = String::new();
  println!("Please type 'y' to confirm the deletion: ");
  std::io::stdin().read_line(&mut input).expect("Failed to read line");
  let input = input.trim();
  if input == "y" || input == "Y" {
      self.task_list.remove(index - 1);
      println!("Task successfully deleted!");
  } else {
      println!("Deletion canceled.");
  }
}
}

fn main () {

  let mut todo_list = ToDoList::new();

  loop {
println!("=== ToDo Handler ===
1. Create
2. Read
3. Update
4. Delete
5. Quit
Choose an option : ");

let mut choice = String::new();
io::stdin().read_line(&mut choice).expect("Failed to read line"); 

match choice.trim().parse::<i32>() {
    Ok(number) => match number {
      1 => {
          println!("Enter task description: ");
          let mut description = String::new();
          io::stdin().read_line(&mut description).expect("Failed to read line");
          let description = description.trim().to_string();
          todo_list.add_task(description);
      }
      2 => {
          println!("Current tasks:");
          todo_list.display_tasks();
      }
      3 => {
          println!("Enter task index to mark as done: ");
          let mut index = String::new();
          io::stdin().read_line(&mut index).expect("Failed to read line");
          let index: usize = index.trim().parse().expect("Invalid input");
          todo_list.mark_done(index - 1);
      }
      4 => {
          println!("Enter task index to delete: ");
          let mut index = String::new();
          io::stdin().read_line(&mut index).expect("Failed to read line");
          let index: usize = index.trim().parse().expect("Invalid input");
          todo_list.delete_task(index);
      }
      5 => break,
      _ => println!("Invalid choice, please try again."),
  }
    Err(_) => println!("Invalid input, please enter a number."),
};
  }
}