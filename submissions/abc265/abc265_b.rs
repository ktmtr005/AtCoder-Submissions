use std::collections::HashMap;
fn main() {
    // input
    let mut sc = Scanner::new(std::io::stdin().lock());
    let (n, m, t) = (sc.next::<usize>(), sc.next::<usize>(), sc.next::<i64>());
    let a: Vec<i64> = (0..n - 1).map(|_x| sc.next::<i64>()).collect();
    let xy: HashMap<usize, i64> = (0..m)
        .map(|_x| (sc.next::<usize>() - 1, sc.next::<i64>()))
        .collect();
    //solve
    let ans = solve(t, a, xy);
    // output
    fastout(if ans { "Yes" } else { "No" });
}
fn solve(mut t: i64, a: Vec<i64>, xy: HashMap<usize, i64>) -> bool {
    for (i, &a_i) in a.iter().enumerate() {
        // i: 部屋番号, a_i: 消費時間
        t -= a_i;
        if t <= 0 {
            return false;
        }
        if let Some(&y) = xy.get(&(i + 1)) {
            t += y;
        }
    }
    true
}
// snippet
struct Scanner<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}
#[allow(dead_code)]
impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Scanner {
            reader,
            buf: Vec::new(),
            pos: 0,
        }
    }
    fn with_capacity(reader: R, capacity: usize) -> Self {
        Scanner {
            reader,
            buf: Vec::with_capacity(capacity),
            pos: 0,
        }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        if self.buf.is_empty() {
            self.read_next_line();
        }
        let mut start = None;
        loop {
            if self.pos == self.buf.len() {
                break;
            }
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self.read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        let elem = unsafe { std::str::from_utf8_unchecked(&self.buf[start.unwrap()..self.pos]) };
        elem.parse()
            .unwrap_or_else(|_| panic!("{}", format!("failed parsing: {}", elem)))
    }
    fn read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
            panic!("Reached EOF");
        }
    }
}
fn fastout(s: &str) {
    use std::io::{stdout, BufWriter, Write};
    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", s).expect("failed to write data.");
}