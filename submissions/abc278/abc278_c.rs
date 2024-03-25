use std::collections::{HashMap, HashSet};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        _n: usize,
        q: usize,
        tab: [(u32, u32, u32); q],
    }
    let ans = solve(q, tab);
    println!(
        "{}",
        ans.iter()
            .map(|&x| if x { "Yes" } else { "No" })
            .collect::<Vec<&str>>()
            .join("\n")
    );
}