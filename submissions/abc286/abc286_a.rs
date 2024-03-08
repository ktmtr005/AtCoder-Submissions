use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: Usize1,
        r: Usize1,
        s: Usize1,
        a: [u32; n],
    }
    let ans = solve(p, q, r, s, &a);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
fn solve(p: usize, q: usize, r: usize, s: usize, a: &[u32]) -> Vec<u32> {
    let p_q = &a[p..=q];
    let r_s = &a[r..=s];
    let mut b = a.to_owned();
    for (i, &v) in p_q.iter().enumerate() {
        b[i + r] = v;
    }
    for (i, &v) in r_s.iter().enumerate() {
        b[i + p] = v;
    }
    b
}