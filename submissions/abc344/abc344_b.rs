fn read_all() -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).ok();
    String::from_utf8(buf).unwrap()
}
fn get_lines(s: String) -> Vec<String> {
    s.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}
fn main() {
    let mut a: Vec<i32> = get_lines(read_all()).iter().map(|x| x.parse().unwrap()).collect();
    a.reverse();
    println!("{}", a.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join("\n"));
}