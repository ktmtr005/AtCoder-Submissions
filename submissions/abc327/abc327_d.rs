use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }
    let ans = solve(n, a, b);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(n: usize, a: Vec<usize>, b: Vec<usize>) -> bool {
    let mut memo = vec![-1; n]; // -1: not visited
    let mut bipartite = true;
    let mut g = vec![Vec::new(); n];
    for (&a_i, &b_i) in a.iter().zip(b.iter()) {
        g[a_i].push(b_i);
        g[b_i].push(a_i);
    }
    for i in 0..n {
        if memo[i] == -1 {
            dfs(i, 0, &g, &mut memo, &mut bipartite);
        }
    }
    bipartite
}
fn dfs(node: usize, n: i32, g: &Vec<Vec<usize>>, memo: &mut Vec<i32>, bipartite: &mut bool) {
    memo[node] = n;
    for &next in &g[node] {
        if memo[next] == -1 {
            dfs(next, 1 - n, g, memo, bipartite);
        } else if memo[next] == memo[node] {
            *bipartite = false;
        }
    }
}