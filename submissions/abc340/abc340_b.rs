use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
        query: [(usize, usize); q],
    }
    println!("{}", solve(&query).iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join("\n"));
}
fn solve(query: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut a = Vec::new();
    let mut ans = Vec::new();
    for &(q, xk) in query {
        if q == 1 {
            a.push(xk);
        }
        else {
            ans.push(a[a.len() - xk]);
        }
    }
    ans
}