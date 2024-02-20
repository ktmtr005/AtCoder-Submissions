use std::collections::BTreeSet;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let repunit = {
        let mut v = [0usize; 12];
        let mut s = String::new();
        for i in 0..12usize {
            s.push('1');
            v[i] = s.parse().expect("Failed to parse.");
        }
        v
    };
    let repunit_trio = {
        let mut s = BTreeSet::new();
        for i in 0..12usize {
            for j in 0..12usize {
                for k in 0..12usize {
                    s.insert(repunit[i] + repunit[j] + repunit[k]);
                }
            }
        }
        s
    };
    let ans = *repunit_trio.iter().skip(n - 1).next().unwrap();
    println!("{}", ans);
}