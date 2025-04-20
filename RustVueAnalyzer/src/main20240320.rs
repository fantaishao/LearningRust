use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use rand::Rng;

fn main() -> io::Result<()> {
    // 从命令行中接收用户输入的 Vue 文件路径
    let vue_file_path = match get_vue_file_path() {
        Some(path) => path,
        None => return Ok(()), // 如果未提供 Vue 文件路径，则退出程序
    };

    // 确定 plugin.js 文件的路径（与 Vue 文件同级）
    let plugin_file_path = vue_file_path.with_file_name("plugin.js");

    // 读取 Vue 文件内容并提取 Three.js 方法名，然后写入 plugin.js 文件
    extract_and_write_threejs_methods(&vue_file_path, &plugin_file_path)?;

    println!("已成功提取 Three.js 方法名并写入 plugin.js 文件。");

    Ok(())
}

/// 从命令行参数获取 Vue 文件路径
fn get_vue_file_path() -> Option<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <Vue_file_path>", args[0]);
        return None;
    }
    let vue_file_path = Path::new(&args[1]).to_path_buf();
    Some(vue_file_path)
}

/// 读取 Vue 文件内容并提取 Three.js 方法名，然后写入 plugin.js 文件
fn extract_and_write_threejs_methods(vue_file_path: &Path, plugin_file_path: &Path) -> io::Result<()> {
    // 打开 Vue 文件并创建读取器
    let vue_file = File::open(vue_file_path)?;
    let reader = BufReader::new(vue_file);

    // 创建 plugin.js 文件并准备写入 Three.js 方法名
    let mut plugin_file = File::create(plugin_file_path)?;

    // 存储提取的 Three.js 方法名
    let mut method_names = Vec::new();

    // 逐行读取 Vue 文件内容，提取 Three.js 方法名并写入 plugin.js 文件
    for line in reader.lines() {
        let line = line?;
        if let Some(index) = line.find("THREE.") {
            if let Some(method_name) = extract_method_name(&line[index + "THREE.".len()..]) {
                method_names.push(method_name.to_string());
            }
        }
    }

    // 写入到 plugin.js 文件中
    for method_name in method_names {
        let mut used_names: HashSet<String> = HashSet::new();
        writeln!(plugin_file, "export const {} = THREE.{};", generate_random_name(&mut used_names), method_name)?;
    }

    // 读取文件内容， 找到从‘three’中引入的内容

    Ok(())
}

/// 从字符串中提取方法名
fn extract_method_name(line: &str) -> Option<&str> {
    line.split_whitespace().next().map(|name| {
        let mut parts = name.split('(');
        parts.next().unwrap_or("").trim_end_matches(';')
    })
}

fn generate_random_name(used_names: &mut HashSet<String>) -> String {
    let mut rng = rand::thread_rng();
    let mut name;
    loop {
        name = rng.clone().sample_iter(rand::distributions::Alphanumeric)
                  .take(8)
                  .map(char::from)
                  .collect::<String>();
        if !used_names.contains(&name) {
            break;
        }
    }
    // 将新生成的随机名称加入到 used_names 中
    used_names.insert(name.clone());
    name
}