use itertools::Itertools;
use proconio::{fastout, input};
use std::vec::Vec;
#[fastout]
fn main() {
    input! {
        n: usize,
        m: i32,
    }
    let ans = solve(n, m);
    println!("{}", ans.iter().map(|v| v.iter().join(" ")).join("\n"));
}
fn solve(n: usize, m: i32) -> Vec<Vec<i32>> {
    (1..=m).combinations(n).collect::<Vec<Vec<i32>>>()
}