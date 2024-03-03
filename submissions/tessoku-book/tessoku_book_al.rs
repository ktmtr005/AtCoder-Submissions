use proconio::{input, fastout};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, u64); n],
    }
    let mut lim = vec![24; d];
    for (l, r, h) in lrh {
        for i in l-1..=r-1 {
            lim[i] = min(lim[i], h);
        }
    }
    let ans = lim.iter().sum::<u64>();
    println!("{}", ans);
}