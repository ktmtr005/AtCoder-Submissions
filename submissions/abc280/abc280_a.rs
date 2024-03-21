use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }
    let ans = solve(s);
    println!("{ans}");
}
fn solve(s: Vec<Vec<char>>) -> usize {
    s.iter()
        .map(|v| v.iter().filter(|&&c| c == '#').count())
        .sum()
}