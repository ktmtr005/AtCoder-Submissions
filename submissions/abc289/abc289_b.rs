use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    }
    let ans = solve(n, a);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
fn solve(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut re = vec![false; n];
    for v in a {
        re[v] = true;
    }
    let mut res = Vec::with_capacity(n);
    let mut stack = Vec::new();
    for (i, &c) in re.iter().enumerate() {
        stack.push(i + 1);
        if !c {
            while let Some(j) = stack.pop() {
                res.push(j);
            }
        }
    }
    res
}