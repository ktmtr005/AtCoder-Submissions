use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let snuke = a | b;
    println!("{snuke}");
}