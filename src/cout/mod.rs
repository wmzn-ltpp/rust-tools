/**
 * 标准输出（带换行）
 * @param {&T} str
 */
pub fn run<T: std::fmt::Debug>(str: &T) {
    println!("{:?}", str);
}
