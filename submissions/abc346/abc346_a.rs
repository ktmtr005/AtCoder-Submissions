use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let ans = solve(a);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
fn solve(a: Vec<u32>) -> Vec<u32> {
    a.windows(2).map(|x| x[0] * x[1]).collect()
}