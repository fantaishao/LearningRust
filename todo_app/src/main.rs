use std::sync::Mutex;
use once_cell::sync::Lazy;

// 定义全局静态变量，使用 Lazy 初始化
static GLOBAL_TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() {
    add_task("learning Rust".to_string());
    list_tasks();
}

fn add_task(description: String) {
    let mut tasks = GLOBAL_TASKS.lock().unwrap();
    tasks.push(description);
}

// 通过索引删除任务
fn remove_task(index: usize) {
    let mut tasks = GLOBAL_TASKS.lock().unwrap();
    tasks.remove(index);
    println!("task at index {} removed", index);
}

fn list_tasks() {
    let tasks = GLOBAL_TASKS.lock().unwrap();
    for task in tasks.iter() {
        println!("{}", task);
    }
}