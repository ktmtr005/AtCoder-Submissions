use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let ans = solve(&s, &t);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(s: &str, t: &str) -> bool {
    s.contains(t)
}