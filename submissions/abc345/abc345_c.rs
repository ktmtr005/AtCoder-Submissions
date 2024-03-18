use std::collections::HashMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = solve(s);
    println!("{ans}");
}
fn solve(s: String) -> u64 {
    let n = s.len() as u64;
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut combination = n * (n - 1) / 2;
    let mut flag_exist_same_char = false;
    for (_c, cnt) in map {
        combination -= cnt * (cnt - 1) / 2;
        if cnt > 1 {
            flag_exist_same_char = true;
        }
    }
    if flag_exist_same_char {
        combination += 1;
    }
    combination
}