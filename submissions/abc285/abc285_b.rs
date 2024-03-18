use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let ans = solve(n, s)
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    println!("{ans}");
}
fn solve(n: usize, s: Vec<char>) -> Vec<usize> {
    let mut ret = vec![0; n - 1];
    for (n, i) in ret.iter_mut().enumerate() {
        for (m, &c1) in s.iter().enumerate() {
            let c2 = s.get(m + n + 1);
            match c2 {
                Some(&c) if c != c1 => continue,
                _ => {
                    *i = m;
                    break;
                }
            }
        }
    }
    ret
}