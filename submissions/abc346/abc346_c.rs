use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n],
    }
    let ans = solve(a, k);
    println!("{ans}");
}
fn solve(mut a: Vec<u64>, k: u64) -> u64 {
    a.sort();
    a.dedup();
    let mut sum = (1 + k) * k / 2;
    for i in a {
        if i <= k {
            sum -= i;
        }
    }
    sum
}