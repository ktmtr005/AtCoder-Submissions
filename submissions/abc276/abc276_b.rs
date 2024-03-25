use std::collections::BTreeSet;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let ans = solve(n, ab);
    println!(
        "{}",
        &ans[1..]
            .iter()
            .map(|v| [
                v.len().to_string(),
                v.iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            ]
            .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
fn solve(n: usize, ab: Vec<(usize, usize)>) -> Vec<BTreeSet<usize>> {
    let mut g = vec![BTreeSet::new(); n + 1];
    for (a, b) in ab {
        g[a].insert(b);
        g[b].insert(a);
    }
    g
}