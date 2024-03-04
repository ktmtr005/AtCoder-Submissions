use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut cnt = [0; 8];
    for i in a {
        cnt[i] += 1;
    }
    if let Some(&i) = cnt.iter().min() {
        println!("{}", i);
    }
}