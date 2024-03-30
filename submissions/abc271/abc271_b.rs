use itertools::Itertools;
fn main() {
    // input
    let mut sc = Scanner::new(std::io::stdin().lock());
    let n: usize = sc.next();
    let q: usize = sc.next();
    let a = (0..n)
        .map(|_x| {
            let l: usize = sc.next();
            (0..l).map(|_x| sc.next::<i32>()).collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let st = (0..q)
        .map(|_x| (sc.next::<usize>() - 1, sc.next::<usize>() - 1))
        .collect::<Vec<(usize, usize)>>();
    // output
    let ans = solve(a, st);
    println!("{}", ans.iter().join("\n"));
}
fn solve(a: Vec<Vec<i32>>, st: Vec<(usize, usize)>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(st.len());
    for (s, t) in st {
        ans.push(a[s][t]);
    }
    ans
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