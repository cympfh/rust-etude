use std::thread;
use std::sync::mpsc;

fn main() {
    let (cc, chan) = mpsc::channel();
    for id in 0..10 {
        let k = cc.clone();
        thread::spawn(move || {
            let _ = k.send(id);
        });
    }
    for _ in 0..10 {
        let x = chan.recv();
        println!("receive {:?}", x); // Ok(id)
    }
    // {
    //     let x = chan.recv(); // .send を待ってしまう (プロセスが終了しない)
    //     println!("wtf: {:?}", x);
    // }
    println!("DONE");
}
