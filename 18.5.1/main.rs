use std::process::{Command,Stdio};
use std::io::prelude::*;

fn main() {
    // process
    let wc = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("{}", why),
            Ok(p) => p
        };

    // stdin of process
    let _ = wc.stdin.unwrap().write_all("HOGEHOGE FUGA".as_bytes());

    // stdout of process
    {
        let mut s = String::new();
        let _ = wc.stdout.unwrap().read_to_string(&mut s);
        println!("{}", s);
    }
}
