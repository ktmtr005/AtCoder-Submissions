use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = solve(s);
    println!("{}", ans);
}
fn solve(s: String) -> u64 {
    const ZERO: u8 = "A".as_bytes()[0] - 1;
    let mut ret = 0;
    for (i, c) in s.chars().rev().collect::<String>().chars().enumerate() {
        let c_u8 = c as u8;
        let n = (c_u8 - ZERO) as u64;
        ret += n * 26_u64.pow(i as u32);
    }
    ret
}