use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    for i in s {
        assert!(i == '1' || i == '0');
        if i == '1' {
            ans += 1;
        }
    }
    println!("{}", ans);
}