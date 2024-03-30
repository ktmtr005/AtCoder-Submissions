use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    }
    let ans = solve(n, p);
    println!("{ans}");
}
fn solve(n: usize, p: Vec<usize>) -> usize {
    let mut g = vec![Vec::new(); n + 1];
    for (i, &v) in p.iter().enumerate() {
        g[v].push(i + 2);
    }
    let mut dp = vec![0; n + 1];
    dfs(1, &g, &mut dp);
    dp[n]
}
fn dfs(node: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    for &i in &g[node] {
        dp[i] = dp[node] + 1;
        dfs(i, g, dp);
    }
}