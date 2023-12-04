#![allow(warnings)]
mod cin;
mod cout;
mod string;
#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let mut str: String = String::from("");
    // 测试输入
    cin::run(&mut str);
    let me: User = User { name: str.clone() };
    cout::run(me);
    // 测试字符串转String数组
    let res: Vec<String> = string::to_vec_string(&str);
    cout::run(res);
    // 测试字符串转i32数组
    let res: Vec<i32> = string::to_vec_number(&str);
    cout::run(res);
}
