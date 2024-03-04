fn read() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read line.");
    buf.trim().to_string()
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
fn main() {
    let (a, b) = {
        let line: Vec<u64> = read().split_whitespace().map(|x| x.parse().unwrap()).collect();
        (line[0], line[1])
    };
    println!("{}", lcm(a, b));
}