use proconio::{fastout, input};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }
    let ans = solve(&a);
    println!("{}", ans);
}
fn solve(a: &Vec<i32>) -> i64 {
    let mut passenger_max = 0_i64;
    let mut passenger_min = 0_i64;
    let mut passenger_diff = 0_i64;
    for &i in a {
        passenger_diff += i as i64;
        passenger_max = max(passenger_max, passenger_diff);
        passenger_min = min(passenger_min, passenger_diff);
    }
    passenger_diff - passenger_min
}