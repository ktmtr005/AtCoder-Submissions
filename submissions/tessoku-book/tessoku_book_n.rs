use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    }
    let ans = solve(k, a, b, c, d);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(k: i64, a: Vec<i64>, b: Vec<i64>, c: Vec<i64>, d: Vec<i64>) -> bool {
    let mut p = Vec::new();
    for &i in &a {
        for &j in &b {
            p.push(i + j);
        }
    }
    let mut q = Vec::new();
    for &i in &c {
        for &j in &d {
            q.push(i + j);
        }
    }
    q.sort();
    for v in p {
        match q.binary_search(&(k - v)) {
            Ok(_i) => return true,
            Err(_i) => continue,
        }
    }
    false
}