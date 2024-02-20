use std::collections::BTreeMap;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let map = {
        let mut map = BTreeMap::new();
        for &i in &a {
            map.entry(i).or_insert(0usize);
            map.insert(i, map.get(&i).unwrap() + i);
        }
        map
    };
    let ans_map = {
        let mut ans = BTreeMap::new();
        let mut sum = 0;
        for (i, v) in map.iter().rev() {
            ans.entry(*i).or_insert(sum);
            sum += *v;
        }
        ans
    };
    let ans = {
        let mut ans = Vec::new();
        for i in &a {
            ans.push(*ans_map.get(i).unwrap());
        }
        ans
    };
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}