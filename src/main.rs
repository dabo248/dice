extern crate rand;
extern crate time;

use rand::Rng;
use std::io;
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    println!("How many dice do you want to throw?");

    let mut dice_count = String::new();

    io::stdin().read_line(&mut dice_count)
        .expect("Failed to read line");

    let dice_count: usize = dice_count.trim().parse()
        .expect("Please type a number!");

    let mut handles = Vec::with_capacity(dice_count);
    let barrier = Arc::new(Barrier::new(dice_count));
    for i in 0..dice_count {
        let c = barrier.clone();
        let counter = i;

        handles.push(thread::spawn(move|| {
            c.wait();

            let number = rand::thread_rng().gen_range(1, 7);
            let now = time::now();
            let seconds = now.to_timespec().sec;
            let nseconds = now.to_timespec().nsec;
            println!("Dice No. {} -> {} ({}.{})", counter, number, seconds, nseconds);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
