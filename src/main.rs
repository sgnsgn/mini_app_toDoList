use uuid::Uuid;
use chrono::{DateTime, Utc};

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

}

fn main() {
    let mut todo_list = ToDoList::new();
    todo_list.add_task("Learn Rust".to_string());
    todo_list.add_task("Build a ToDo app".to_string());

    println!("Current tasks:");
    todo_list.display_tasks();

    let task_index = 1;
    todo_list.mark_done(task_index);

    println!("\nUpdated tasks:");
    todo_list.display_tasks();  
}