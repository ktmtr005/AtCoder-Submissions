use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i64,
    }
    let mut ans = n % 998244353;
    if ans < 0 {
        ans += 998244353;
    }
    println!("{ans}");
}