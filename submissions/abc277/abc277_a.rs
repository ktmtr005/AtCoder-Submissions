use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: u32,
        p: [u32; n],
    }
    let ans = p.iter().position(|&n| n == x).unwrap();
    println!("{}", ans + 1);
}