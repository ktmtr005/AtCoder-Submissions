use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let ans = solve(s).join("\n");
    println!("{ans}");
}
fn solve(mut s: Vec<String>) -> Vec<String> {
    s.reverse();
    s
}