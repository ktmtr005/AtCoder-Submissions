use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
    }
    for i in n..=999 {
        let n_1 = i % 10;
        let n_10 = (i / 10) % 10;
        let n_100 = i / 100;
        if n_100 * n_10 == n_1 {
            println!("{}", i);
            break;
        }
    }
}