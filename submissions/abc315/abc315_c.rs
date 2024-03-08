use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        fs: [(u32, u32); n],
    }
    let ans = solve(fs);
    println!("{}", ans);
}
fn solve(mut fs: Vec<(u32, u32)>) -> u32 {
    fs.sort_by(|a, b| a.1.cmp(&b.1));
    let partial_satisfaction = fs.pop().unwrap();
    let satisfaction_different_flavor = fs
        .iter()
        .filter(|&x| x.0 != partial_satisfaction.0)
        .last()
        .unwrap_or(&(0, 0));
    let satisfaction_same_flavor = fs
        .iter()
        .filter(|&x| x.0 == partial_satisfaction.0)
        .last()
        .unwrap_or(&(0, 0));
    (partial_satisfaction.1 + satisfaction_different_flavor.1)
        .max(partial_satisfaction.1 + satisfaction_same_flavor.1 / 2)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let fs = vec![(1, 4), (2, 10), (2, 8), (3, 6)];
        assert_eq!(solve(fs), 16);
        let fs2 = vec![(1, 4), (2, 10)];
        assert_eq!(solve(fs2), 14);
        let fs3 = vec![(1, 4), (1, 8)];
        assert_eq!(solve(fs3), 10);
    }
}