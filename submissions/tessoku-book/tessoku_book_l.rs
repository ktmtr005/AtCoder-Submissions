use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
        a: [i32; n],
    }
    let ans = solve(k, &a);
    println!("{}", ans);
}
fn solve(k: i32, a: &Vec<i32>) -> i32 {
    let mut left = 1;
    let mut right = 10_i32.pow(9);
    while left < right {
        let center = (left + right) / 2;
        if check(center, k, a) == true {
            right = center;
        }
        else {
            left = center + 1;
        }
    }
    left
}
fn check(t: i32, k: i32, a: &Vec<i32>) -> bool {
    let mut sum: i64 = 0;
    for &i in a {
        sum += (t / i) as i64;
    }
    if sum >= k as i64 {
        return true;
    }
    else {
        return false;
    }
}