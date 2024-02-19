use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: Chars,
        s: [Chars; h],
    }
    let mut ans = 0usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut pos_i = i;
            let mut pos_j = j;
            let mut ok = true;