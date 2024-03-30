use std::collections::HashMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = solve(&s);
    println!("{ans}");
}
fn solve(s: &str) -> u32 {
    let next_saturday: HashMap<&str, u32> =
        ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]
            .into_iter()
            .zip([5, 4, 3, 2, 1])
            .collect();
    *next_saturday.get(s).unwrap()
}