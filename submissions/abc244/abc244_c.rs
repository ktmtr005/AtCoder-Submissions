use std::collections::BTreeSet;
fn main() {
    let n = {
        let mut sc = Scanner::new(std::io::stdin().lock());
        sc.next::<i32>()
    };
    solve(n);
}
fn solve(n: i32) {
    let mut said = (1..=2 * n + 1).collect::<BTreeSet<i32>>();
    loop {
        let takahashi = said.pop_first().unwrap();
        println!("{takahashi}");
        let aoki = {
            let mut sc = Scanner::new(std::io::stdin().lock());
            sc.next::<i32>()
        };
        if aoki == 0 {
            return;
        }
        said.remove(&aoki);
    }
}
// snippet
struct Scanner<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}
impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Scanner {
            reader,
            buf: Vec::new(),
            pos: 0,
        }
    }
    #[allow(dead_code)]
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