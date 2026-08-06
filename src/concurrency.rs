use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

// --- spawn thread --- //
#[test]
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread:  {} ", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("main thread : {}", i);
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap();
}

// --- scope -- //
#[test]
fn main2() {
    let a = [1, 2, 3];
    thread::scope(|scope| {
        scope.spawn(|| {
            for x in &a {
                println!("x : {}", x);
            }
        });
    });
}

// --- move ---- //
#[test]
fn main3() {
    let mut a = [1, 2, 3];
    let handle = thread::spawn(move || {
        for x in a {
            println!("x : {}", x);
        }
    });
    a[0] = 42;
    handle.join().unwrap();
}

// --- Arc<T> --- //

#[test]
fn main4() {
    let a = Arc::new([1, 2, 3]);
    let mut handles = Vec::new();
    for i in 0..2 {
        let arc = Arc::clone(&a);
        handles.push(thread::spawn(move || {
            println!("Thread: {} {arc:?} ", i,);
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}

// --- Arc<T>, Mutex<T> --- //
#[test]
fn main5() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // MutexGuard dropped here — lock released automatically
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
    // Output: Final count: 5
}
