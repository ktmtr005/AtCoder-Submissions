use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
        d: usize,
    }
    println!("{}", solve(a, b, d).iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}
fn solve(a: i32, b: i32, d: usize) -> Vec<i32> {
    let mut ans = Vec::new();
    for i in (a..=b).step_by(d) {
        ans.push(i);
    }
    ans
}