use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i64,
        x: i64,
        a: [i64; n]
    }
    let ans = solve(x, &a);
    println!("{}", ans);
}
fn solve(x: i64, a: &Vec<i64>) -> i64 {
    let mut left: usize = 0;
    let mut right: usize = a.len() - 1;
    let mut ans = 0;
    while left <= right {
        let center = (left + right) / 2;
        if a[center] < x {
            left = center + 1;
        }
        if x < a[center] {
            right = center - 1;
        }
        if a[center] == x {
            ans = center as i64 + 1;
            break;
        }
    }
    ans
}