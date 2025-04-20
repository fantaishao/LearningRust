use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

fn main() -> io::Result<()> {
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
            return Ok(());
        }
    };

    // 创建一个文件读取器
    let reader = io::BufReader::new(file);

    // Store mappings of original names to random names
    let mut rename_map: HashMap<String, String> = HashMap::new();
    
    // Generate random names
    let random_names = [
        "object1", "object2", "object3", // Add more random names as needed
    ];

    // 遍历文件的每一行
    for line in reader.lines() {
        let line = line?;
        // if let Ok(line) = line {
            // 检查该行是否包含“THREE.”
            if let Some(index) = line.find("THREE.") {
                // 如果包含，则打印该行
                let old_name = &line[index..(index + "THREE.".len())];
                let new_name = random_names.choose(&mut rand::thread_rng()).unwrap();
                rename_map.insert(old_name.to_string(), new_name.to_string());
                println!("{} {}", &old_name, &new_name)
            }
        // }
    }

    let mut plugin_file = File::create("plugin.js")?;
    for (old_name, new_name) in rename_map {
        writeln!(plugin_file, "export const {} = {};", new_name, old_name)?;
    }

    Ok(())
}
