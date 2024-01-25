use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>, //存储工作线程的信息
    sender: Option<mpsc::Sender<Job>>, //用于将工作提交给线程池
}

// Job 结构用于表示工作单元，它是一个闭包，实现了 FnOnce、Send 和 'static 特性
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender),
        }
    }

    // 执行工作单元的方法
    // execute 方法用于将工作提交给线程池，其中 F 是闭包类型
    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

// Worker 结构用于表示工作线程的信息，包括线程的标识和 JoinHandle
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// 工作线程实现
// 外部调用者无需知道Worker的存在，所以使用私有的声明
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { 
            id, 
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}