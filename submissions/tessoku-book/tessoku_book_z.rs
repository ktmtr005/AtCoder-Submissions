use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        q: usize,
        x: [usize; q],
    }
    let is_prime = sieve_of_eratosthenes(300000);
    for i in x {
        if is_prime[i] == true {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}
fn sieve_of_eratosthenes(end: usize) -> Vec<bool> {
    let end_sqrt = {
        let mut i = 1usize;
        while i * i < end {
            i += 1;
        }
        i
    };
    let mut is_prime = vec![true; end + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 1..end_sqrt {
        if is_prime[i] == true {
            for j in (i*2..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}