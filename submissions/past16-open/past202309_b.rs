use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32
    }
    let ans = a == b && a == c && b == c;
    println!("{}", if ans == true {"Yes"} else {"No"});
}