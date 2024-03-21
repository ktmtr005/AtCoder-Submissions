use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ans = solve(n);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );
}
fn solve(n: usize) -> Vec<usize> {
    (0..=n).rev().collect::<Vec<usize>>()
}