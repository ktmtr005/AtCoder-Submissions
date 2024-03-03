use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: u64,
        a: [u64; n],
        c: [u64; m],
    }
    let a_all = a.iter().sum::<u64>() * c.len() as u64;
    let b_all = b * (a.len() * c.len()) as u64;
    let c_all = c.iter().sum::<u64>() * a.len() as u64;
    let ans = a_all + b_all + c_all;
    println!("{}", ans);
}