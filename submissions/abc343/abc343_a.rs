use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let ans = (a + b + 1) % 10;
    println!("{}", ans);
}