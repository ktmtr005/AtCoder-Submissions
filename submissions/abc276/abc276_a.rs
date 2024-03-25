use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    match solve(s) {
        Some(i) => println!("{}", i + 1),
        None => println!("-1"),
    }
}
fn solve(s: Vec<char>) -> Option<usize> {
    s.iter().rposition(|&c| c == 'a')
}