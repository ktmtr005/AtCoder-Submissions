use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        query: [(i64, i64); n],
    }
    for i in &query {
        let ans = i.0 + i.1;
        println!("{}", ans);
    }
}