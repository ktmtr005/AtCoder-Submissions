use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        y: i32,
    }
    let ans = solve(y);
    println!("{ans}");
}
fn solve(mut y: i32) -> i32 {
    while y % 4 != 2 {
        y += 1;
    }
    y
}