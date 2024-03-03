use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let ans = palindromic_cube_number(n);
    println!("{}", ans);
}
fn palindromic_cube_number(n: u64) -> String {
    let mut num = 1;
    let mut num_3 = num * num * num;
    let mut ans = String::from("");
    while num_3 <= n {
        let s = num_3.to_string();
        if s == s.chars().rev().collect::<String>() {
            ans = s;
        }
        num += 1;
        num_3 = num * num * num;
    }
    ans
}
#[cfg(test)]
mod test {
    use std::time;
    use super::*;
    #[test]
    fn it_works() {
        let now = time::Instant::now();
        palindromic_cube_number(1_000_000_000_000_000_000_u64);
        println!("{:?}", now.elapsed());
    }
}