use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut bi = vec![0u64; n];
    let mut bj = bi.clone();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                bi[i] += 1;
                bj[j] += 1;
            }
        }
    }
    let mut ans = 0u64;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                ans += (bi[i] - 1) * (bj[j] - 1);
            }
        }
    }
    println!("{}", ans);
}