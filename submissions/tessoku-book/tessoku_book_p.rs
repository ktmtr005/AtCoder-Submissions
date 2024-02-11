use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n - 1],
        mut b: [i32; n - 2],
    }
    for _ in 0..2 {
        a.insert(0, 0);
        b.insert(0, 0);
    }
    b.insert(0, 0);
    let ans = solve(n, &a, &b);
    println!("{}", ans);
}
fn solve(n: usize, a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut dp = vec![0; n + 1];
    dp[2] = a[2];
    for i in 3..=n {
        dp[i] = min(dp[i - 1] + a[i], dp[i - 2] + b[i]);
    }
    dp[n]
}