use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let distance = 2 * n - 2;
    let ans = distance <= k && (k - distance) % 2 == 0;
    println!("{}", if ans == true {"Yes"} else {"No"});
}