use std::collections::HashSet;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: u32,
        m: usize,
    }
    let mut set = Vec::new();
    for _i in 0..m {
        input! {
            c: usize,
            a: [u32; c],
        }
        let s: HashSet<u32> = a.into_iter().collect();
        set.push(s);
    }
    let ans = solve(n, m, set);
    println!("{ans}");
}
fn solve(n: u32, m: usize, set: Vec<HashSet<u32>>) -> usize {
    let pattern: u32 = 1 << m;
    let mut cnt = 0;
    'next_pattern: for i in 0..pattern {
        for x in 1..=n {
            let mut get_success = false;
            for (j, s) in set.iter().enumerate() {
                if ((i >> j) & 1) == 1 && s.get(&x).is_some() {
                    get_success = true;
                }
            }
            if !get_success {
                continue 'next_pattern;
            }
        }
        cnt += 1;
    }
    cnt
}