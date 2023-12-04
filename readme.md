# Rust Tools

> Rust 工具库

## 模块构成

- mod cin
  - fn run():String 标准输入
- mod cout
  - fn run(&String)
- mod string
  - fn to_vec_string(&String): Vec<String> 将字符串转成字符串数组
  - fn to_vec_number<T: std::str::FromStr>(str: &String) -> Vec<T> where T::Err: std::fmt::Debug, 将字符串转成数字 Vec 数组，具体类型为接收返回值的数组类型
