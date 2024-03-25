use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: u64,
        k: u64,
    }
    let ans = solve(x, k);
    println!("{ans}");
}
fn solve(mut x: u64, k: u64) -> u64 {
    for _i in 0..k {
        x += x % 10;
        x /= 10;
    }
    for _i in 0..k {
        x *= 10;
    }
    x
}