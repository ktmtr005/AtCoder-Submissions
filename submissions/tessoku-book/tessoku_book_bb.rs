use std::collections::HashMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut result_map = HashMap::new();
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                x: String,
                y: usize,
            }
            result_map.entry(x).or_insert(y);
        }
        else if query_type == 2 {
            input! {
                x: String,
            }
            println!("{}", *result_map.get(&x).unwrap());
        }
    }
}