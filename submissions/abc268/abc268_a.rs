use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
    }
    let ans = solve(vec![a, b, c, d, e]);
    println!("{ans}");
}
fn solve(mut arr: Vec<u32>) -> usize {
    arr.sort();
    arr.dedup();
    arr.len()
}