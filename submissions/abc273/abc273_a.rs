use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let ans = solve(n);
    println!("{ans}");
}
fn solve(k: u32) -> u32 {
    if k == 0 {
        return 1;
    }
    k * solve(k - 1)
}