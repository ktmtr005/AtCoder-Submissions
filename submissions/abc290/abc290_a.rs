use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [Usize1; m],
    }
    let mut ans = 0;
    for i in b {
        ans += a[i];
    }
    println!("{}", ans);
}