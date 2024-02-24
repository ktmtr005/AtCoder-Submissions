use im_rc::HashSet;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }
    let pattern_num = n * (n - 1) / 2;
    let mut not_discord = HashSet::new();
    for i in 0..a.len() {
        for j in 1..a[0].len() {
            let (x, y) = (a[i][j], a[i][j - 1]);
            not_discord.insert((x.min(y), x.max(y)));
        }
    }
    let ans = pattern_num - not_discord.len();
    println!("{}", ans);
}