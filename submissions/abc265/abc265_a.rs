use std::cmp::min;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: i32,
        y: i32,
        n: i32,
    }
    let ans = solve(x, y, n);
    println!("{ans}");
}
fn solve(x: i32, y: i32, n: i32) -> i32 {
    let mut price = i32::MAX;
    for (i, y_i) in (0..=n).step_by(3).enumerate() {
        let j = n - y_i;
        price = min(price, i as i32 * y + j * x);
    }
    price
}