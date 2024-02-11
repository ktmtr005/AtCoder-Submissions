use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let ans = solve(n);
    println!("{}", ans);
}
fn solve(mut n: i32) -> i32 {
    let mut ans = 0;
    let mut cnt = 0;
    while n > 0 {
        ans += (n % 10) * 2_i32.pow(cnt);
        cnt += 1;
        n = n / 10;
    }
    ans