use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() {
    // 提示用户输入文件地址
    println!("请输入文件的地址：");

    // 创建一个可变的字符串变量来存储用户输入的文件地址
    let mut file_path = String::new();

    // 从标准输入读取用户输入的文件地址
    io::stdin().read_line(&mut file_path)
        .expect("无法读取文件地址");

    // 去除用户输入中可能的换行符或空白字符
    let file_path = file_path.trim();

    // 尝试打开用户输入的文件
    let file = match File::open(&file_path) {
        // Ok(mut file) => {
        //     // 创建一个可变的字符串变量来存储文件内容
        //     let mut content = String::new();

        //     // 读取文件内容并存储到字符串变量中
        //     match file.read_to_string(&mut content) {
        //         Ok(_) => println!("文件内,容： \n{}", content),
        //         Err(e) => eprintln!("无法读取文件内容：{}", e),
        //     }
        // },
        // Err(e) => eprintln!("无法打开文件: {}", e),
        Ok(file) => file,
        Err(_) => {
            eprintln!("无法打开文件");
            return;
        }
    };

    // 创建一个文件读取器
    let reader = io::BufReader::new(file);

    // 遍历文件的每一行
    for line in reader.lines() {
        if let Ok(line) = line {
            // 检查该行是否包含“THREE.”
            if line.contains("THREE.") {
                // 如果包含，则打印该行
                println!("{}", line)
            }
        }
    }
}
