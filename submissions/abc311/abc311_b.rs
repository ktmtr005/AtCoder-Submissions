use std::cmp::max;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: i32,
        _d: i32,
        s: [Chars; n],
    }
    println!("{}", solve(&s));
}
fn solve(s: &Vec<Vec<char>>) -> i32 {
    let mut hima = vec![true; s[0].len()];
    for h in s {
        for (i, &v) in h.iter().enumerate() {
            if v == 'x' {
                hima[i] = false;
            }
        }
    }
    let mut ans = 0;
    let mut partial_hima = 0;
    for i in hima {
        if i == false {
            partial_hima = 0;
        }
        else {
            partial_hima += 1;
            ans = max(ans, partial_hima);
        }
    }
    ans
}