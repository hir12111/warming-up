struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next_line(&self) -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Fail to read");
        line
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let line = self.next_line();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let arr: Vec<usize> = scan.next_n(n);
    if n <= 2 {
        println!("{}", n);
        return;
    }
    let mut maxcount = 2usize;
    let mut count = 2usize;
    for i in 2..n {
        if arr[i] == arr[i - 1] + arr[i - 2] {
            count += 1;
            maxcount = maxcount.max(count);
        } else {
            count = 2;
        }
    }
    println!("{}", maxcount);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
