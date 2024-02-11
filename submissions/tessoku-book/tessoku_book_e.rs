use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let ans = solve(n, k);
    println!("{}", ans);
}
fn solve(n: i32, k: i32) -> i32{
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j < k  && k - (i + j) <= n {
                ans += 1;
            }
        }
    }
    ans
}