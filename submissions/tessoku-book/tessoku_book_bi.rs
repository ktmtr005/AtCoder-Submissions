use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n + 1];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 1..g.len() {
        println!("{}: {{{}}}", i, g[i].iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(", "));
    }
}