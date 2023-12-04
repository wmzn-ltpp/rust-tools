pub fn run() -> String {
    let mut str: String = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    str
}
