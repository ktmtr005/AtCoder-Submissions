use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        m: u32,
    }
    let ans = if 4 <= m && m <= 9 {
        "Yes"
    }
    else {
        "No"
    };
    println!("{}", ans);
}