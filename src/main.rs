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
    let mut list_index = 0;
	for task in self.task_list.iter() {
        println!("{}. [✗] {}", list_index, task.description);
        list_index += 1;
    }   
}
}

fn main() {
    let mut todo_list = ToDoList::new();
    todo_list.add_task("Learn Rust".to_string());
    todo_list.add_task("Build a ToDo app".to_string());

    println!("Current tasks:");
    todo_list.display_tasks();
}