use std::thread;
use std::time::Duration;

fn main() {
    // take 0+1+2+3+4+5+6 sec
    for i in 0..7 {
        let child = thread::spawn(move || {
            thread::sleep(Duration::new(i, 0)); // i sec
            println!("id:{}", i);
        });
        let _ = child.join();
    }
}

