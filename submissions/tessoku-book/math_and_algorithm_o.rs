use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let ans = gcd(a, b);
    println!("{}", ans);
}
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}