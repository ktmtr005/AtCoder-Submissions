use proconio::input;
fn dec_sum(n: i32) -> i32 {
    let mut num = n;
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num = num / 10;
    }
    sum
}
fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }
    let mut ans = 0;