
use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

fn race_counter(){
    let mut race_counter =0;
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                race_counter +=1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Race counter result: {}",  race_counter);
}

fn static_counter(){
    static mut STATIC_COUNTER: i32 = 0;
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    STATIC_COUNTER +=1;
                }

            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe{
        println!("Static counter result: {}", STATIC_COUNTER);
    }
}

fn mutex_counter(){
    let locker = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let now = Instant::now();
    for _ in 0..100 {
        let clone = locker.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100000 {
                let mut cnt = clone.lock().unwrap();
                *cnt += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = now.elapsed().as_millis();

    let mut counter = locker.lock().unwrap();
    println!("Mutex counter result: {} ({} ms)", *counter, elapsed);
}
fn atomic_counter(){
    let mut counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let now = Instant::now();
    for _ in 0..100 {
        let mut cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100000 {
                cnt.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = now.elapsed().as_millis();

    println!("Atomic counter result: {} ({} ms)", counter.load(Ordering::SeqCst), elapsed);
}

fn main() {
    race_counter();
    static_counter();
    mutex_counter();
    atomic_counter();

}