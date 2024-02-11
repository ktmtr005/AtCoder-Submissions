// abc338_c
use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: i64,
        q: [i64; n],
        a: [i64; n],
        b: [i64; n],
    }
    let ans = solve(n, &q, &a, &b);
    println!("{}", ans);
}
fn solve(n: i64, q: &Vec<i64>, a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    let mut ans = 0;
    let q_max = q.iter().max().unwrap();
    const INF: i64 = 10_i64.pow(9);
    for x in 0..=*q_max {
        let mut y = INF;
        for i in 0..n as usize {
            if q[i] < a[i] * x {
                y = -INF;
            }
            else if b[i] > 0 {
                y = y.min((q[i] - a[i] * x) / b[i]);
            }
        }
        ans = max(ans, x + y);
    }
    ans
}