use std::sync::mpsc;
use std::thread;

fn calc(num: i32) -> i32 {
    let mut result = 0;
    for i in 1..=num {
        result += i;
    }
    result
}

fn main() {
    let request_nums = [55550, 10000, 30];

    let (tx, rx) = mpsc::channel::<(i32, i32)>();

    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = calc(num);
            sender.send((num, answer)).unwrap();
        });
    }

    let mut job = request_nums.len();

    loop {
        if let Ok((num, answer)) = rx.recv() {
            job -= 1;
            println!("num is {},calc answer is {}", num, answer);
        }
        if job <= 0 {
            break;
        }
    }
}
