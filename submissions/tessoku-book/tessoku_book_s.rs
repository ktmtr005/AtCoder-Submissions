use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(i64, i64); n],
    }
    let ans = solve(n, w, &wv);
    println!("{}", ans);
}
fn solve(n: usize, w: usize, wv: &Vec<(i64, i64)>) -> i64 {
    const INF: i64 = 1_000_000_000_000_000;
    let mut dp = vec![vec![-INF; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..dp.len() {
        for j in 0..dp[0].len() {
            if j < wv[i - 1].0 as usize {
                dp[i][j] = dp[i - 1][j];
            }
            else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 1][j - wv[i - 1].0 as usize] + wv[i - 1].1);
            }
        }
    }
    *dp.iter().last().unwrap().iter().max().unwrap()
}