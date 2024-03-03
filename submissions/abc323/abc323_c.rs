use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; m],
        s: [Chars; n],
    }
    let mut now_scores = {
        let mut score = Vec::new();
        for (i, v) in s.iter().enumerate() {
            let mut sum = i as u64 + 1;
            for (idx, &val) in v.iter().enumerate() {
                if val == 'o' {
                    sum += a[idx];
                }
            }
            score.push(sum);
        }
        score
    };
    let mx = *now_scores.iter().max().unwrap();
    for i in 0..n {
        let mut remain = Vec::new();
        for (idx, &val)  in s[i].iter().enumerate() {
            if val == 'x' {
                remain.push(a[idx]);
            }
        }
        remain.sort();
        remain.reverse();
        let mut ans = 0;
        while now_scores[i] < mx {
            now_scores[i] += remain[ans];
            ans += 1;
        }
        println!("{}", ans);
    }
}