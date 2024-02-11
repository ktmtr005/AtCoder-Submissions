use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
    }
    println!("{:010b}", n);
}