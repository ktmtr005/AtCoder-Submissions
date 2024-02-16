use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }
    let mut cnt = 0;
    let mut ans = Vec::new();
    for i in s {
        if i == 'o' {
            if cnt >= k {
                ans.push('x');
            }
            else {
                ans.push('o');
            }
            cnt += 1;
        }
        else {
            ans.push('x');
        }
    }
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(""));
}