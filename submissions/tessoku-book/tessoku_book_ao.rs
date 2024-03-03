use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = false;
    for i in 0..n-2 {
        if (s[i], s[i+1], s[i+2]) == ('R', 'R', 'R') || (s[i], s[i+1], s[i+2]) ==  ('B', 'B', 'B') {
            ans = true;
        }
    }
    println!("{}", if ans == true {"Yes"} else {"No"});
}