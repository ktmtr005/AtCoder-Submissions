use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }
    let (k, p) = solve(n, &a, &b);
    println!("{}\n{}", k, p.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}
fn solve(n: usize, a: &Vec<usize>, b: &Vec<usize>) -> (usize, Vec<usize>) {
    let mut dp = vec![0; n + 1];
    dp[2] = a[0];
    for i in 3..dp.len() {
        dp[i] = min(dp[i - 1] + a[i - 2], dp[i - 2] + b[i - 3]);
    }
    let mut place = n;
    let mut p = Vec::new();
    loop {
        p.push(place);
        if place == 1 {
            break;
        }
        if dp[place - 1] + a[place - 2] == dp[place] {
            place -= 1;
        }
        else {
            place -= 2;
        }
    }
    p.reverse();
    (p.len(), p)
}