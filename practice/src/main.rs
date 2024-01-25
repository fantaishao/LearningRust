use std::env;

//  两数之和
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <number1> <number2>", args[0]);
        return;
    }
    
    // 在命令行中，通过用户输入的参数都会被解析为字符串。
    // 如果你需要将这些字符串转换为整数，你必须使用 parse 方法来进行转换
    let x: i32 = args[1].parse().expect("Failed to parse number1");
    let y: i32 = args[2].parse().expect("Failed to parse number1");

    // let (x, y) = (1, 2);
    let s = sum(x, y);

    // assert_eq!(s, 3);
    println!("两数之和为 {}", s);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}