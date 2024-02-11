use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32
    }
    println!("{}", solve(a, b));
}
fn solve(a: u32, b: u32) -> u32 {
    a.pow(b) + b.pow(a)
}