use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{:<02X}", n);
}