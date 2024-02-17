use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let mut ans = Vec::new();
    for i in 0..2*n+1 {
        if i % 2 == 0 {
            ans.push('1');
        }
        else {
            ans.push('0');
        }
    }
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(""));
}