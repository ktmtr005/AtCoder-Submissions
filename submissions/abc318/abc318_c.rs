use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        f: [u64; n],
    }
    let ans = solve(d, p, f);
    println!("{}", ans);
}
fn solve(d: usize, p: usize, mut f: Vec<u64>) -> u64 {
    f.sort();
    let f_cumsum: Vec<u64> = f.iter().scan(0, |cum, x| {
        *cum += x;
        Some(*cum)
    }).collect();
    let k = (f.len()+d-1) / d;
    let mut ret = (p * k) as u64;
    {
        let mut i = 0;
        while let Some(j) = (f.len() - 1).checked_sub(i * d) {
            ret = ret.min(f_cumsum[j] + (p * i) as u64);
            i += 1;
        }
    }
    ret
}