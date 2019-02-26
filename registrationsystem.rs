use std::collections::HashMap;

struct Scan {
    buffer: std::collections::VecDeque<String>
}

impl Scan {
    fn new() -> Scan {
        Scan { buffer: std::collections::VecDeque::new() }
    }

    fn next<T: std::str::FromStr>(&mut self)-> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("Fail to read");
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }

}

fn _main() {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut names: HashMap<String, usize> = HashMap::new();
    for _ in 0..n {
        let name: String = scan.next();
        let v = names.entry(name.clone())
            .and_modify(|x| { *x+=1; })
            .or_insert(0);
        if *v == 0 {
            println!("OK");
        } else {
            println!("{}{}", name, v);
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1<<23).spawn(_main).unwrap().join().unwrap();
}
