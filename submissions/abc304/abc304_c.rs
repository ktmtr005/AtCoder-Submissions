use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        d: i32,
        xy: [(i32, i32); n],
    }
    let ans = solve(n, d, xy)
        .iter()
        .map(|&x| if x == true { "Yes" } else { "No" })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{ans}");
}
fn solve(n: usize, d: i32, xy: Vec<(i32, i32)>) -> Vec<bool> {
    let mut g = vec![Vec::new(); n];
    for (i, (x1, y1)) in xy.iter().enumerate() {
        for (j, (x2, y2)) in xy.iter().enumerate() {
            let dist_2 = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
            if dist_2 <= d * d {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let mut memo = vec![false; n];
    dfs(0, &g, &mut memo);
    memo
}
fn dfs(node: usize, g: &Vec<Vec<usize>>, memo: &mut [bool]) {
    memo[node] = true;
    for &i in &g[node] {
        if memo[i] == false {
            dfs(i, g, memo);
        }
    }
}