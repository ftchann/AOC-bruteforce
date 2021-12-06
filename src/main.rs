use std::sync::{Arc, Mutex};
use std::thread;

// use thread_id;
fn main() {
    let numbers: [u8; 300] = [2,5,5,3,2,2,5,1,4,5,2,1,5,5,1,2,3,3,4,1,4,1,4,4,2,1,5,5,3,5,4,3,4,1,5,4,1,5,5,5,4,3,1,2,1,5,1,4,4,1,4,1,3,1,1,1,3,1,1,2,1,3,1,1,1,2,3,5,5,3,2,3,3,2,2,1,3,1,3,1,5,5,1,2,3,2,1,1,2,1,2,1,2,2,1,3,5,4,3,3,2,2,3,1,4,2,2,1,3,4,5,4,2,5,4,1,2,1,3,5,3,3,5,4,1,1,5,2,4,4,1,2,2,5,5,3,1,2,4,3,3,1,4,2,5,1,5,1,2,1,1,1,1,3,5,5,1,5,5,1,2,2,1,2,1,2,1,2,1,4,5,1,2,4,3,3,3,1,5,3,2,2,1,4,2,4,2,3,2,5,1,5,1,1,1,3,1,1,3,5,4,2,5,3,2,2,1,4,5,1,3,2,5,1,2,1,4,1,5,5,1,2,2,1,2,4,5,3,3,1,4,4,3,1,4,2,4,4,3,4,1,4,5,3,1,4,2,2,3,4,4,4,1,4,3,1,3,4,5,1,5,4,4,4,5,5,5,2,1,3,4,3,2,5,3,1,3,2,2,3,1,4,5,3,5,5,3,2,3,1,2,5,2,1,3,1,1,1,5,1];
    let mut globalcounter = 0;
    let threads = 8;
    let times = (numbers.len() + (threads - 1))/threads;

    for i in 0..times {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for j in 0..threads {

            let counter = Arc::clone(&counter);
            let index = i*threads + j;
            if index == numbers.len() {
                break;
            }
            let num = numbers[index];
            let handle = thread::spawn(move || {
                let mut fishes = vec![num];
                for _ in 0..256 {
                    for j in 0..fishes.len() {
                        if fishes[j] == 0 {
                            fishes[j] = 6;
                            fishes.push(8);
                        } else {
                            fishes[j] -= 1;
                        }
                    }
                }
                let mut num = counter.lock().unwrap();
                *num += fishes.len();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let count = *counter.lock().unwrap();
        globalcounter += count;
    }
    println!("{}", globalcounter);
}
