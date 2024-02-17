use num_integer::lcm;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let nm_lcm = lcm(n, m);
    // 答えで二分探索
    let mut left = 0usize;
    let mut right = std::usize::MAX / 2;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let num = div_num(n, m, nm_lcm, mid); // n, m のうちちょうど一方のみで割り切れる数のうち小さい方から(num)番目
        if num < k {
            left = mid;
        }
        else {
            right = mid;
        }
    }
    println!("{}", right);
}
// n, m のいずれかで割れるものの個数
fn div_num(n: usize, m: usize, nm_lcm: usize, v: usize) -> usize {
    let a1 = v / n;
    let a2 = v / m;
    let a3 = v / nm_lcm;
    a1 + a2 - 2 * a3
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn div_num_works() {
        assert_eq!(div_num(3, 5, 15, 30), 14);
    }
}