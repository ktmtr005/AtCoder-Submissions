use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        k: usize,
    }
    let ans = solve(k);
    println!("{ans}");
}
fn solve<'a>(k: usize) -> &'a str {
    const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    &ABC[..k]
}