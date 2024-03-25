use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let ans = h
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index + 1)
        .unwrap();
    println!("{ans}");
}