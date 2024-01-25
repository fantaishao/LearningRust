use std::{
    fs,
    io::{prelude::*, BufReader}, // 读取和写入数据，BufReader可以实现缓冲区读取
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    // 创建一个 TcpListener 并绑定到地址 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 创建一个具有 4 个线程的线程池
    let pool = ThreadPool::new(4);

    // 接受传入的连接并交给线程池处理
    for stream in listener.incoming().take(2) {
        let _stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(_stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    // 使用 BufReader 读取连接中的数据
    let buf_reader = BufReader::new(&mut stream);
    // 从请求的第一行获取请求信息
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // 根据请求信息生成响应信息
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET / sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // 读取文件内容
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // 构建响应字符串
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // 将响应写入连接
    // 用as_bytes将字符串转换为字节数组
    stream.write_all(response.as_bytes()).unwrap();    
}
