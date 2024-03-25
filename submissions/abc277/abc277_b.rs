use std::collections::HashSet;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let ans = solve(s);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(s: Vec<String>) -> bool {
    let mut used = HashSet::new();
    for v in s {
        let [c1, c2, ..] = v.chars().collect::<Vec<char>>()[..] else {unreachable!()};
        if !['H', 'D', 'C', 'S'].contains(&c1) {
            return false;
        }
        if ![
            'A', 'T', 'J', 'Q', 'K', '2', '3', '4', '5', '6', '7', '8', '9',
        ]
        .contains(&c2)
        {
            return false;
        }
        if !used.insert((c1, c2)) {
            return false;
        }
    }
    true
}