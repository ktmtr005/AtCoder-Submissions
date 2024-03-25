use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }
    let ans = solve(ab);
    println!("{ans}");
}
fn solve(ab: Vec<(u64, u64)>) -> u64 {
    let mut g = HashMap::new();
    let mut not_visited = HashSet::new();
    for (a, b) in ab {
        g.entry(a).or_insert(Vec::new()).push(b);
        g.entry(b).or_insert(Vec::new()).push(a);
        not_visited.insert(a);
        not_visited.insert(b);
    }
    let depth_max: Cell<u64> = Cell::new(1);
    dfs(1, &g, &mut not_visited, &depth_max);
    depth_max.get()
}
fn dfs(
    node: u64,
    g: &HashMap<u64, Vec<u64>>,
    not_visited: &mut HashSet<u64>,
    depth_max: &Cell<u64>,
) {
    if let Some(v) = g.get(&node) {
        not_visited.remove(&node);
        for &i in v {
            if not_visited.contains(&i) {
                depth_max.set(depth_max.get().max(i));
                dfs(i, g, not_visited, depth_max);
            }
        }
    }
}