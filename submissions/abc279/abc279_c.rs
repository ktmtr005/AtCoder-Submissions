use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }
    let ans = solve(w, s, t);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(w: usize, s: Vec<Vec<char>>, t: Vec<Vec<char>>) -> bool {
    let mut s_transposed = (0..w)
        .map(|i| s.iter().map(|v| v[i]).collect::<String>())
        .collect::<Vec<String>>();
    let mut t_transposed = (0..w)
        .map(|i| t.iter().map(|v| v[i]).collect::<String>())
        .collect::<Vec<String>>();
    s_transposed.sort();
    t_transposed.sort();
    s_transposed == t_transposed
}