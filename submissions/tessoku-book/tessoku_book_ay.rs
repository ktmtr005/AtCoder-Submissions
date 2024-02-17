use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut book_stack = Vec::new();
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                title: String,
            }
            book_stack.push(title);
        }
        else if query_type == 2 {
            println!("{}", *book_stack.iter().last().unwrap());
        }
        else if query_type == 3 {
            book_stack.pop();
        }
    }
}