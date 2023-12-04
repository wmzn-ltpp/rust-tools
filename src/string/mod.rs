/**
 * 字符串转字符串数组
 */
pub fn to_vec_string(str: &String) -> Vec<String> {
    let res: Vec<String> = str
        .split_whitespace()
        .map(|tem_str| tem_str.to_string())
        .collect();
    res
}

/**
 * 字符串转成数字类型的Vec数组
 */
pub fn to_vec_number<T: std::str::FromStr>(str: &String) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let res: Vec<T> = str
        .split_whitespace()
        .map(|tem_str| tem_str.parse().unwrap())
        .collect();
    res
}
