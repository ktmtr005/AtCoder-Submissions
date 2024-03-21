use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }
    let ans = solve(&s);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
fn solve(s: &[i64]) -> Vec<i64> {
    [&[0], s].concat().windows(2).map(|x| x[1] - x[0]).collect()
}