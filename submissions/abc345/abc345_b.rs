use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: i64,
    }
    let ans = solve(x);
    println!("{ans}");
}
fn solve(x: i64) -> i64 {
    if x >= 0 {
        (x + 9) / 10
    } else {
        x / 10
    }
}