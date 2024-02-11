use proconio::{fastout, input};
use proconio::marker::Chars;
#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    match solve(&n) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn solve(n: &Vec<char>) -> bool {
    let mut is_321_like = true;
    for i in 1..n.len() {
        if n[i] >= n[i - 1] {
            is_321_like = false;
        }
    }
    is_321_like
}