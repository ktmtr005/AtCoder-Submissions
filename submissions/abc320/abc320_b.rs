use proconio::{fastout, input};
use proconio::marker::Chars;
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    println!("{}", solve(&s));
}
fn is_parindrome(s: &Vec<char>) -> bool {
    let mut is_parindrome = true;
    let n = s.len();
    for i in 0..n {
        if s[i] != s[n - 1 - i] {
            is_parindrome = false;
        }
    }
    is_parindrome
}
fn solve(s: &Vec<char>) -> usize {
    let n = s.len();
    let mut ans = 1;
    for i in 0..n {
        for j in i + 1..=n {
            let mut partial_chars = Vec::new();
            for k in i..j {
                partial_chars.push(s[k]);
            }
            if is_parindrome(&partial_chars) == true {
                ans = max(ans, j - i);
            }
        }
    }
    ans
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(is_parindrome(&vec!['A', 'D', 'A']), true);
        assert_eq!(is_parindrome(&vec!['A', 'D', 'D']), false);
    }
}