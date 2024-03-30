use itertools::Itertools;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let ans = solve(n);
    println!("{}", ans.iter().join("\n"));
}
fn solve(n: u64) -> Vec<u64> {
    let n_bit: Vec<(usize, char)> = format!("{n:b}")
        .chars()
        .rev()
        .enumerate()
        .filter(|(_, x)| *x == '1')
        .collect();
    let mut ans = vec![0];
    for i in 1..=n_bit.len() {
        for v in (0..n_bit.len()).combinations(i) {
            ans.push(v.iter().map(|&x| 1 << n_bit[x].0).sum::<u64>());
        }
    }
    ans.sort();
    ans
}