use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n + 1];
    let mut visited = vec![false; n + 1];
    visited[0] = true;
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    dfs(1, &g, &mut visited);
    let ans = visited.iter().all(|&x| x == true);
    if ans == true {
        println!("The graph is connected.");
    }
    else {
        println!("The graph is not connected.");
    }
}
fn dfs(pos: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[pos] = true;
    for &v in &g[pos] {
        let nex = v;
        if visited[nex] == false {
            dfs(nex, g, visited);
        }
    }
}