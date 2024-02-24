use proconio::{input, fastout};
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        mut a: [u64; n],
    }
    a.sort();
    a.push(9_000_000_000_000_000);
    let mut ans = 0u64;
    let mut r = 0usize;
    for l in 0..n {
        while a[r] < a[l] + m {
            r += 1;
        }
        ans = max(ans, (r - l) as u64);
    }
    println!("{}", ans);
}