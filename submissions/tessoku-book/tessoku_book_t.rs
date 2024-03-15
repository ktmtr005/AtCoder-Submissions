use proconio::{fastout, input, marker::Chars};
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let ans = solve(s, t);
    println!("{ans}");
}
fn solve(s: Vec<char>, t: Vec<char>) -> i32 {
    let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
    for (i, &c2) in t.iter().enumerate() {
        for (j, &c1) in s.iter().enumerate() {
            if c1 == c2 {
                dp[i + 1][j + 1] = max(dp[i][j] + 1, max(dp[i][j + 1], dp[i + 1][j]));
            } else {
                dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }
    *dp.iter().last().unwrap().iter().last().unwrap()
}