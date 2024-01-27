use std::env;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

// 定义任务结构体
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

// 创建一个结构体来管理任务
struct TaskManager {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        // Check if the id already exists
        while self.tasks.contains_key(&self.next_id) {
            println!("Conflict! ID {} already exists, trying the next one.", self.next_id);
            self.next_id += 1;
        }

        let task = Task::new(self.next_id, description);
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        self.save_tasks();
    }

    fn update_task(&mut self, id: usize, new_description: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.description = new_description;
            self.save_tasks();
            println!("Task with ID {} updated.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn delete_task(&mut self, id:usize) {
        if let Some(task) = self.tasks.get_mut(&id) {
            self.tasks.remove(&id);
            self.save_tasks();
            println!("Task with ID {} is deleted.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = true;
            self.save_tasks();
            println!("Task with ID {} is completed.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn list_tasks(&self) {
        for task in self.tasks.values() {
            println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
        }
    }

    fn save_tasks(&self) {
        let tasks_json = serde_json::to_string(&self.tasks).expect("Failed to serialize tasks");
        println!("Saving tasks: {}", tasks_json);
        if let Ok(_file) = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("tasks.json") 
        {
            fs::write("tasks.json", tasks_json).expect("Failed to write tasks to file");
        } else {
            eprintln!("Failed to open tasks.json for writing");
        }
    }

    fn load_tasks(&mut self) {
        if let Ok(file) = fs::read_to_string("tasks.json") {
            if let Ok(tasks) = serde_json::from_str(&file) {
                self.tasks = tasks;
            } else {
                eprintln!("Failed to deserialize tasks from file");
            }
        } else {
            eprintln!("Failed to read tasks.json");
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    task_manager.load_tasks(); // 加载任务

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <command> args");
        return;
    }

    let command = &args[1];
    
    match command.as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Usage cargo run add <description>");
                return;
            }
            let description = args[2].clone();
            println!("Adding task with description: {}", description);
            task_manager.add_task(description);
        }
        "update" => {
            if args.len() < 4 {
                println!("Usage: cargo run update <id> <new_description>");
                return;
            }

            let id:usize = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid ID format.");
                    return;
                }
            };
            let new_description = args[3].clone();
            task_manager.update_task(id, new_description);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: cargo run delete <id>");
                return;
            }
            let id:usize = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid ID format.");
                    return;
                }
            };
            task_manager.delete_task(id);
        }
        "complete" => {
            if args.len() < 3 {
                println!("Usage: cargo run complete <id>");
                return;
            }
            let id:usize = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid ID format.");
                    return;
                }
            };
            task_manager.complete_task(id);
        }
        "list" => {
            task_manager.list_tasks();
        }
        _ => {
            // 处理其他情况，也可以选择不处理
            println!("Unknown command: {}", command);
        }
    }
}
