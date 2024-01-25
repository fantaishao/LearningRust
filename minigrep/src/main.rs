use std::env; // :: 在 Rust 中用于表示路径，指定访问特定项的位置
use std::process; // 模块提供了创建和管理子进程的功能

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 对build返回的result进行处理
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，
    // 如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；
    // 如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);


    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
