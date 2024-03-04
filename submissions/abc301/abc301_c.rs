use std::collections::BTreeMap;
use std::cmp::max;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut s_cnt = BTreeMap::new();
    for c in s {
        *s_cnt.entry(c).or_insert(0) += 1;
    }
    let mut t_cnt = BTreeMap::new();
    for c in t {
        *t_cnt.entry(c).or_insert(0) += 1;
    }
    for c in "atcoder".chars() {
        let m = max(*s_cnt.entry(c).or_insert(0), *t_cnt.entry(c).or_insert(0));
        if *s_cnt.entry('@').or_insert(0) < m - *s_cnt.get(&c).unwrap() || *t_cnt.entry('@').or_insert(0) < m - *t_cnt.get(&c).unwrap() {
            println!("No");
            return;
        }
        *s_cnt.entry('@').or_insert(0) -= m - *s_cnt.get(&c).unwrap();
        s_cnt.insert(c, m);
        *t_cnt.entry('@').or_insert(0) -= m - *t_cnt.get(&c).unwrap();
        t_cnt.insert(c, m);
    }
    println!("{}", if s_cnt == t_cnt {"Yes"} else {"No"})
}