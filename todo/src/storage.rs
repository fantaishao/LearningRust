// storage.rs
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::task::TaskManager;
use serde_json;
use std::fs;
use std::collections::HashMap;

// pub fn save_tasks(task_manager: &TaskManager) {
//     if let Ok(tasks_json) = serde_json::to_string(&task_manager.tasks) {
//         if let Ok(_file) = fs::write("tasks.json", tasks_json) {
//             return;
//         }
//     }
//     eprintln!("Failed to save tasks to file");
// }

// pub fn load_tasks(task_manager: &mut TaskManager) {
//     if let Ok(file_content) = fs::read_to_string("tasks.json") {
//         if let Ok(tasks) = serde_json::from_str::<HashMap<usize, Task>>(&file_content) {
//             task_manager.tasks = tasks;
//             task_manager.next_id = task_manager.tasks.keys().max().map_or(0, |&x| x) + 1;
//             return;
//         }
//     }
//     eprintln!("Failed to load tasks from file");
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub description_input: String,
    pub tasks: Vec<TaskManager>,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    pub fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Iced", "Todos")
            {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or_default()
            };

            path.push("todos.json");

            path
    }
    pub async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }
}
