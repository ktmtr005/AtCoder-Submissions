use proconio::{fastout, input};
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        r: i32,
        c: i32,
    }
    if max((r - 8).abs(), (c - 8).abs()) % 2 == 0 {
        println!("white");
    } else {
        println!("black");
    }
}