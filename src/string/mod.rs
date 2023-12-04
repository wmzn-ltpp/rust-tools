use std::string;

/**
 * 字符串转字符串数组
 * @param {&String} str
 * @return {Vec<String>}
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
 * @param {&String} str
 * @return {Vec<T>}
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

/**
 * 根据下标修改String
 * @param {&mut String} str
 * @param {usize} loc
 * @param {char} new_val
 */
pub fn update_string_loc_val(str: &mut String, loc: usize, new_val: char) {
    let mut chars: Vec<char> = str.chars().collect();
    if let Some(ch) = chars.get_mut(loc) {
        *ch = new_val;
    }
    *str = chars.into_iter().collect();
}
