下面代码报错：error[E0618]: expected function, found `Job`；call expression requires function
main.rs
use std::{
    fs,
    io::{prelude::*, BufReader}, // 读取和写入数据，BufReader可以实现缓冲区读取
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    // 监听地址： 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        pool::execute(|| {
            handle_connection(_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET / sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // 用as_bytes将字符串转换为字节数组
    stream.write_all(response.as_bytes()).unwrap();    
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

}

lib.rs
use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

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

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// 外部调用者无需知道Worker的存在，所以使用私有的声明
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}