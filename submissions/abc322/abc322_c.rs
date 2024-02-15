use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    }
    println!("{}", solve(n, &a).iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join("\n"));
}
fn solve(n: usize, a: &Vec<usize>) -> Vec<i32> {
    let mut fireworks = vec![-1; n];
    for &i in a {
        fireworks[i] = 0;
    }
    let mut cnt = 0;
    for i in (0..fireworks.len()).rev() {
        cnt += 1;
        if fireworks[i] < 0 {
            fireworks[i] = cnt;
        }
        else {
            cnt = 0;
        }
    }
    fireworks
}