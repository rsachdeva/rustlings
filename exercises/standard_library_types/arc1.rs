// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and creating an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Scroll down for hints :)

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
//    println!("numbers is {:?}", numbers);
//    let numbers_iter = numbers.iter();
//    println!("numbers sum is {:?}", numbers_iter.sum::<u32>());
    let shared_numbers = Arc::new(Mutex::new(numbers));
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let shared_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(
        thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            let child_numbers = shared_numbers.lock().unwrap();
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    let total_threads = joinhandles.len();
    println!("total theads are same as number of offset which is staring point incremented by 5 each time used for summation {:?}", total_threads);
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}




















// Make `shared_numbers` be an `Arc` from the numbers vector. Then, in order
// to avoid creating a copy of `numbers`, you'll need to create `child_numbers`
// inside the loop but still in the main thread.

// `child_numbers` should be a clone of the Arc of the numbers instead of a
// thread-local copy of the numbers.
