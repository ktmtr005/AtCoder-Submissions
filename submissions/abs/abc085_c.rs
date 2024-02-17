use proconio::{input, fastout};
#[fastout]
fn main() {
    input!{
        n: i32,
        y: i32,
    }
    let mut ans = [-1, -1, -1];
    for i in 0..=n {
        for j in 0..=n-i {
            let k = n - i - j;
            if 1000 * i + 5000 * j + 10000 * k == y {
                ans = [k, j, i];
            }
        }
    }
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}