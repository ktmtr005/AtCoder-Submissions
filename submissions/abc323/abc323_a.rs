use proconio::{fastout, input};
use proconio::marker::Chars;
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = solve(&s);
    match ans {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn solve(s: &Vec<char>) -> bool {
    let mut ans = true;
    for i in (1..s.len()).step_by(2) {
        if s[i] != '0' {
            ans = false;
        }
    }
    ans
}