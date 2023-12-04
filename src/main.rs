#![allow(warnings)]
use std::error::Error;

mod cin;
mod cout;
mod string;

#[derive(Debug)]
struct User {
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    cout::run(&"请输入完成后回车");
    // 测试输入
    let mut str = cin::run();
    cout::run(&"读入数据：");
    // 测试输出
    cout::run(&str);
    // 测试输出
    let me: User = User { name: str.clone() };
    cout::run(&me);
    // 测试字符串转String数组
    str = String::from("a ab abc abcd");
    let res: Vec<String> = string::to_vec_string(&str);
    cout::run(&res);
    // 测试字符串转i32数组
    str = String::from("1 10 100 1000");
    let res: Vec<i32> = string::to_vec_number(&str);
    cout::run(&res);
    // 测试字符串转i64数组
    let res: Vec<i64> = string::to_vec_number(&str);
    cout::run(&res);
    // 测试下标修改String
    string::update_string_loc_val(&mut str, 0, '1');
    cout::run(&"修改后的读入数据：");
    cout::run(&str);
    Ok(())
}
