use proconio::{fastout, input};
use proconio::marker::Chars;
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        _n: i32,
        m: i32,
        s: Chars,
    }
    let ans = solve(m, &s);
    println!("{}", ans);
}
fn solve(m: i32, s: &Vec<char>) -> i32 {
    let mut muji = 0;
    let mut buy = 0;
    let mut logo = 0;
    for  &i in s {
        if i == '0' {
            buy = max(buy, logo);
            (muji, logo) = (0, 0);
        }
        if i == '1' {
            if muji < m {
                muji += 1;
            }
            else {
                logo += 1;
            }
        }
        if i == '2' {
            logo += 1;
        }
    }
    max(buy, logo)
}