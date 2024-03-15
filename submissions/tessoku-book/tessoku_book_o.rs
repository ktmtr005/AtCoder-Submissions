use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let ans = solve(a);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
fn solve(a: Vec<i32>) -> Vec<usize> {
    let mut t = a.clone();
    t.sort();
    t.dedup();
    let mut b = Vec::with_capacity(a.len());
    for v in a {
        let i = t.binary_search(&v).unwrap();
        b.push(i + 1);
    }
    b
}