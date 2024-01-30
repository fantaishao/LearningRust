use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    TaskUpdate,
    DescriptionEdited(String),
    // FinishEdition,
    TaskDeleted,
}

// 定义任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskManager {
    pub description: String,
    pub completed: bool,

    #[serde(skip)]
    state: TaskState,
}

impl TaskManager {
    pub fn new(description: String) -> Self {
        // TaskManager {
        //     tasks: HashMap::new(),
        //     next_id: 1,
        // }
        TaskManager {
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::TaskUpdate => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::TaskDeleted => {}
        }
    }

    // pub fn add_task(&mut self, description: String) {
    //     // Check if the id already exists
    //     while self.tasks.contains_key(&self.next_id) {
    //         println!("Conflict! ID {} already exists, trying the next one.", self.next_id);
    //         self.next_id += 1;
    //     }

    //     let task = Task::new(self.next_id, description);
    //     self.tasks.insert(self.next_id, task);
    //     self.next_id += 1;
    //     self.save_tasks();
    // }

    // pub fn update_task(&mut self, id: usize, new_description: String) {
    //     if let Some(task) = self.tasks.get_mut(&id) {
    //         task.description = new_description;
    //         self.save_tasks();
    //         println!("Task with ID {} updated.", id);
    //     } else {
    //         println!("Task with ID {} not found.", id);
    //     }
    // }

    // pub fn delete_task(&mut self, id:usize) {
    //     if let Some(task) = self.tasks.get_mut(&id) {
    //         self.tasks.remove(&id);
    //         self.save_tasks();
    //         println!("Task with ID {} is deleted.", id);
    //     } else {
    //         println!("Task with ID {} not found.", id);
    //     }
    // }

    // pub fn complete_task(&mut self, id: usize) {
    //     if let Some(task) = self.tasks.get_mut(&id) {
    //         task.completed = true;
    //         self.save_tasks();
    //         println!("Task with ID {} is completed.", id);
    //     } else {
    //         println!("Task with ID {} not found.", id);
    //     }
    // }

    // pub fn list_tasks(&self) {
    //     for task in self.tasks.values() {
    //         println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
    //     }
    // }

    // pub fn save_tasks(&self) {
    //     let tasks_json = serde_json::to_string(&self.tasks).expect("Failed to serialize tasks");
    //     println!("Saving tasks: {}", tasks_json);
    //     if let Ok(_file) = fs::OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .truncate(true)
    //         .open("tasks.json") 
    //     {
    //         fs::write("tasks.json", tasks_json).expect("Failed to write tasks to file");
    //     } else {
    //         eprintln!("Failed to open tasks.json for writing");
    //     }
    // }

    // pub fn load_tasks(&mut self) {
    //     if let Ok(file) = fs::read_to_string("tasks.json") {
    //         if let Ok(tasks) = serde_json::from_str(&file) {
    //             self.tasks = tasks;
    //         } else {
    //             eprintln!("Failed to deserialize tasks from file");
    //         }
    //     } else {
    //         eprintln!("Failed to read tasks.json");
    //     }
    // }
}