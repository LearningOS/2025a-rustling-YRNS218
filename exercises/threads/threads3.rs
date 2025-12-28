// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn send_tx(q: Vec<u32>, tx: mpsc::Sender<u32>) {
    // 克隆 q 给第一个线程
    let q1 = q.clone();
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in &q1[0..q1.len() / 2] {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 克隆 q 给第二个线程
    let q2 = q.clone();
    let tx2 = tx.clone();
    thread::spawn(move || {
        for val in &q2[q2.len() / 2..] {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let q = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    send_tx(q, tx);

    // 接收线程发送的值
    for received in rx {
        println!("Got: {}", received);
    }
}