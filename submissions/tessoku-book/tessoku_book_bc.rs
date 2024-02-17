use std::collections::BTreeSet;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut card_set = BTreeSet::new();
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                x: i32,
            }
            card_set.insert(x);
        }
        else if query_type == 2 {
            input! {
                x: i32,
            }
            card_set.remove(&x);
        }
        else if query_type == 3 {
            input! {
                x: i32,
            }
            if let Some(&v) = card_set.range(x..).next() {
                println!("{}", v);
            }
            else {
                println!("-1");
            }
        }
    }
}