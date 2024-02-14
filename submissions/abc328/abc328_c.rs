use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        _n: usize,
        q: usize,
        s: Chars,
        query: [(usize, usize); q],
    }
    let ans = solve(&s, &query);
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join("\n"));
}
fn solve(s: &Vec<char>, query: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut accum = vec![0; s.len() + 1];
    let mut char_before = '-';
    for (i, &c) in s.iter().enumerate() {
        if c == char_before {
            accum[i + 1] = accum[i] + 1;
        }
        else {
            accum[i + 1] = accum[i];
        }
        char_before = c;
    }
    let mut ret = Vec::new();
    for &(l, r) in query {
        ret.push(accum[r] - accum[l]);
    }
    ret
}