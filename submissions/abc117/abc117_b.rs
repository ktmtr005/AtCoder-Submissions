use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        l: [u32; n],
    }
    let ans = solve(l);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(mut l: Vec<u32>) -> bool {
    l.sort();
    let longest = l.pop().unwrap();
    longest < l.iter().sum::<u32>()
}