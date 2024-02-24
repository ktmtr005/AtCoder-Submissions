use std::collections::BTreeSet;
use proconio::fastout;
fn read() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().split_whitespace().map(|s| s.parse().expect("Failed to parse.")).collect()
}
#[fastout]
fn main() {
    let &n = read().get(0).unwrap();
    let mut p = Vec::new();
    let mut c = Vec::new();
    let mut f = Vec::new();
    for _i in 0..n {
        let product = read();
        p.push(product[0]);
        c.push(product[1]);