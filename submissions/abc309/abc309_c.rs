use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        ab: [(u64, u64); n],
    }
    let ans = solve(k, ab);
    println!("{}", ans);
}
fn solve(k: u64, mut ab: Vec<(u64, u64)>) -> u64 {
    ab.sort();
    let mut sum = ab.iter().fold(0, |pre, x| pre + x.1);
    if sum <= k {
        return 1;
    }
    for (a, b) in ab {
        sum -= b;
        if sum <= k {
            return a + 1;
        }
    }
    0
}
#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_solve() {
        let mut rnd = rand::thread_rng();
        for _i in 0..10 {
            let k = rnd.gen_range(0..=1_000_000_000);
            println!("{}", k);
            let ab: Vec<(u64, u64)> = (0..100u64)
                .map(|_x| {
                    (
                        rnd.gen_range(1..=1_000_000_000),
                        rnd.gen_range(1..=1_000_000_000),
                    )
                })
                .collect();
            let ans = solve(k, ab);
            println!("{}", ans);
        }
    }
}