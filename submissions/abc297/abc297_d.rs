use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let ans = solve(a, b);
    println!("{ans}");
}
fn solve(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a.max(b), a.min(b));
    let mut ans = 0;
    while b > 0 {
        ans += a / b;
        a %= b;
        (a, b) = (b, a);
    }
    ans - 1
}