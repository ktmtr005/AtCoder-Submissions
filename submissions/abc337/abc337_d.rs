use proconio::{fastout, input, marker::Chars};
use std::cmp::{min, max};
#[fastout]
fn main() {
    input! {
        h: usize,
        _w: usize,
        k: usize,
        s: [Chars; h],
    }
    let ans = solve(k, s);
    match ans {
        Some(i) => println!("{}", i),
        None => println!("-1"),
    }
}
fn solve(k: usize, s: Vec<Vec<char>>) -> Option<u64> {
    let w = s[0].len();
    let h = s.len();
    let mut ret = std::u64::MAX;
    let mut x = vec![0u64; max(h, w) + 1];
    let mut d = x.clone();
    for row in 0..h {
        for col in 0..w {
            x[col+1] = x[col];
            d[col+1] = d[col];
            if s[row][col] == 'x' {
                x[col+1] += 1;
            }
            if s[row][col] == '.' {
                d[col+1] += 1;
            }
        }
        if let Some(n) = w.checked_sub(k) {
            for i in 1..=n+1 {
                if x[i+k-1].checked_sub(x[i-1]) == Some(0) {
                    ret = min(ret, d[i+k-1] - d[i-1]);
                }
            }
        }
    }
    for col in 0..w {
        for row in 0..h {
            x[row+1] = x[row];
            d[row+1] = d[row];
            if s[row][col] == 'x' {
                x[row+1] += 1;
            }
            if s[row][col] == '.' {
                d[row+1] += 1;
            }
        }
        if let Some(n) = h.checked_sub(k) {
            for i in 1..=n+1 {
                if x[i+k-1].checked_sub(x[i-1]) == Some(0) {
                    ret = min(ret, d[i+k-1] - d[i-1]);
                }
            }
        }
    }
    if ret as usize > k {
        None
    }
    else {
        Some(ret)
    }
}