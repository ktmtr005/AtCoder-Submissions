use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n * 7],
    }
    println!("{}", solve(&a).iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}
fn solve(a: &Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for i in (0..a.len()).step_by(7) {
        let mut b = 0;
        for j in i..i+7 {
            b += a[j];
        }
        ans.push(b);
    }
    ans
}