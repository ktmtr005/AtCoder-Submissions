use std::collections::VecDeque;
use proconio::{fastout, input, marker::{Chars, Usize1}};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [Usize1; n],
    }
    let ans = solve(m, s, c);
    println!("{}", ans);
}
fn solve(m: usize, s: Vec<char>, c: Vec<usize>) -> String {
    let mut colored_subsequence = vec![VecDeque::new(); m];
    for (&character, &color) in s.iter().zip(c.iter()) {
        colored_subsequence[color].push_back(character);
    }
    for i in 0..m {
        let last = colored_subsequence[i].pop_back().unwrap();
        colored_subsequence[i].push_front(last);
    }
    let mut ret = String::new();
    for i in c {
        ret.push(colored_subsequence[i].pop_front().unwrap());
    }
    ret
}