use std::collections::BTreeMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let ans = solve(n, a);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );
}
fn solve(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut map = BTreeMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut ans = vec![0; n];
    for (i, (_k, &v)) in map.iter().rev().enumerate() {
        ans[i] = v;
    }
    ans
}