use std::collections::HashSet;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let ans = solve(s);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(s: Vec<char>) -> bool {
    let (mut x, mut y) = (0, 0);
    let mut set = HashSet::from([(x, y)]);
    for c in s {
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => unreachable!(),
        }
        match set.get(&(x, y)) {
            Some(_t) => return true,
            None => {
                set.insert((x, y));
            }
        }
    }
    false
}