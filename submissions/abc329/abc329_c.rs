use std::collections::HashMap;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    let ans = solve(&s);
    println!("{}", ans);
}
fn solve(s: &Vec<char>) -> i32 {
    let mut count_chars = HashMap::new();
    let mut same_chars_count = 0;
    let mut char_before = s[0];
    for &i in s {
        count_chars.entry(i).or_insert(0);
        if i != char_before {
            same_chars_count = 0;
            char_before = i;
        }
        same_chars_count += 1;
        count_chars.insert(i, std::cmp::max(*count_chars.get(&i).unwrap(), same_chars_count));
    }
    let mut ret = 0;
    for (_k, v) in count_chars {
        ret += v;
    }
    ret
}