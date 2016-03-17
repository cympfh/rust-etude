use std::io;
use std::str::FromStr;
use std::cmp::{min,max};
use std::f64::consts::PI;
use std::collections::HashMap;

fn main() {
    let mut sc = Scanner::new();
    let mut book = HashMap::new();
    loop {
        let c = sc.get_char();
        match c {
            '.' => break,
            '!' => {
                let name: String = sc.cin();
                let num: String = sc.cin();
                book.insert(name.clone(), num.clone());
            },
            '?' => {
                let name: String = sc.cin();
                match book.get(&name) {
                    Some(num) => println!("{}'s number is {}", name, num),
                    None => println!("Not found for {}", name),
                }
            },
            _ => {}
        }
    }

}

struct Scanner { stdin: io::Stdin, buffer: Vec<String>, }
impl Scanner {
    fn new() -> Scanner { Scanner { stdin: io::stdin(), buffer: Vec::new() } }
    fn reserve(&mut self) {
        if self.buffer.len() == 0 {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split(char::is_whitespace).map(String::from) {
                if !w.is_empty() { self.buffer.push(w) }
            }
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        self.reserve();
        match self.buffer[0].parse::<T>() {
            Ok(a) => { self.buffer.remove(0); a },
            Err(_) => panic!("parse err"),
        }
    }
    fn get_char(&mut self) -> char {
        self.reserve();
        let head = self.buffer[0].chars().nth(0).unwrap();
        let tail = String::from( &self.buffer[0][1..] );
        if tail.len() > 0 {
            self.buffer[0] = tail
        } else {
            self.buffer.remove(0);
        }
        head
    }
}
