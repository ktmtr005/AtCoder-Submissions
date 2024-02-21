use proconio::{input, fastout};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        d: i64,
    }
    let x_max = (d as f64).sqrt().ceil() as i64;
    let mut ans = std::i64::MAX;
    for x in 0..x_max {
        let c = x * x - d;
        let tmp_ans = if c >= 0 {
            c
        }
        else {
            let y_floor = (-c as f64).sqrt() as i64;
            let y_ceil = (-c as f64).sqrt().ceil() as i64;
            min((y_floor * y_floor + c).abs(), (y_ceil * y_ceil + c).abs())
        };
        ans = min(ans, tmp_ans);
    }
    println!("{}", ans);
}