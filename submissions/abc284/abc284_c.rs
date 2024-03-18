use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let ans = solve(n, uv);
    println!("{ans}");
}
fn solve(n: usize, uv: Vec<(usize, usize)>) -> usize {
    let mut g = vec![Vec::new(); n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut cnt = 0;
    while let Some(i) = visited.iter().position(|&x| !x) {
        visited[i] = true;
        dfs(i, &g, &mut visited);
        cnt += 1;
    }
    cnt
}
fn dfs(node: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    for &i in &g[node] {
        if !visited[i] {
            visited[i] = true;
            dfs(i, g, visited);
        }
    }
}