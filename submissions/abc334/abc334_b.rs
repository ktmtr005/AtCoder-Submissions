use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }
    println!("{}", solve(a, m, l, r));
}
fn solve(a: i64, m: i64, l: i64, r: i64) -> i64 {
    floor(r - a, m) - floor(l - a - 1, m)
}
fn floor(x: i64, m: i64) -> i64 {
    let r = (x % m + m) % m;
    (x - r) / m
}