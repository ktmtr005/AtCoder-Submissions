use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: usize,
        _m: usize,
        s: [Chars; n],
    }
    let ans = solve(s);
    println!("{ans}");
}
fn solve(s: Vec<Vec<char>>) -> usize {
    let mut cnt = 0;
    'next: for (a, b) in (0..s.len()).tuple_combinations() {
        for (&c1, &c2) in s[a].iter().zip(s[b].iter()) {
            if c1 == 'x' && c2 == 'x' {
                continue 'next;