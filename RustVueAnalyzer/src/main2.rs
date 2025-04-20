use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use serde_json;

fn main() -> io::Result<()> {
    // 从命令行中接收用户输入的 Vue 文件路径
    let vue_file_path = match get_vue_file_path() {
        Some(path) => path,
        None => return Ok(()), // 如果未提供 Vue 文件路径，则退出程序
    };

    // 创建新文件 plugin.js
    let plugin_file_path = vue_file_path.with_file_name("plugin.js");
    let mut plugin_file = match File::create(&plugin_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("无法创建 plugin.js 文件: {:?}", e);
            return Err(e);
        }
    };

    // 读取 Vue 文件内容
    let vue_file = File::open(&vue_file_path)?;
    let vue_reader = BufReader::new(vue_file);

    // 存储提取的方法名，避免重复
    let mut method_names = HashSet::new();

    // 遍历 Vue 文件的每一行
    for line in vue_reader.lines() {
        let line = line?;
        // 检查该行是否包含 "THREE."
        if let Some(index) = line.find("THREE.") {
            // 获取方法名部分
            let method_name = line[index + 6..]
                .split(|c: char| !c.is_alphanumeric())
                .next()
                .unwrap_or_default();

            // 将方法名写入 plugin.js
            if !method_name.is_empty() && !method_names.contains(method_name) {
                let random_var = generate_random_var();
                writeln!(plugin_file, "export const {} = THREE.{};", random_var, method_name)?;
                method_names.insert(method_name.to_string());
            }
        }
    }

    println!("已成功提取 Three.js 方法名并写入 plugin.js 文件。");

    Ok(())
}

/// 生成一个随机变量名
fn generate_random_var() -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    (0..6)
        .map(|_| *charset.choose(&mut rng).unwrap() as char)
        .collect()
}

/// 从命令行参数获取 Vue 文件路径
fn get_vue_file_path() -> Option<std::path::PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <Vue_file_path>", args[0]);
        return None;
    }
    let vue_file_path = Path::new(&args[1]).to_path_buf();
    Some(vue_file_path)
}
