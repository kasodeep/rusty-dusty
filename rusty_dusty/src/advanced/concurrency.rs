use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use threadpool::ThreadPool;

fn basic_threads_example() {
    println!("\n=== Basic Threads ===");
    
    // spawn a new thread using spawn.
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("Spawned thread counting: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=3 {
        println!("Main thread counting: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // wait for spawned thread to finish.
    handle.join().unwrap();
}

fn channel_example() {
    println!("\n=== Channels ===");
    
    // create a channel.
    let (sender, receiver) = mpsc::channel();
    
    // clone sender for multiple producer threads.
    let sender2 = sender.clone();

    thread::spawn(move || {
        sender.send("Hello from thread 1").unwrap();
    });

    thread::spawn(move || {
        sender2.send("Hello from thread 2").unwrap();
    });

    for _ in 0..2 {
        println!("Received: {}", receiver.recv().unwrap());
    }
}

fn shared_state_example() {
    println!("\n=== Shared State (Arc and Mutex) ===");
    
    // create thread-safe counter using Arc and Mutex.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} modified counter to: {}", i, *num);
        });
        handles.push(handle);
    }

    // wait for all threads to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

fn rwlock_example() {
    println!("\n=== RwLock Example ===");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4]));
    let mut handles = vec![];

    // spawn reader threads.
    for i in 0..2 {
        let data_clone = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            // Multiple threads can read simultaneously
            let values = data_clone.read().unwrap();
            println!("Reader {} sees: {:?}", i, *values);
        }));
    }

    // spawn writer thread.
    let data_clone = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        let mut values = data_clone.write().unwrap();
        values.push(5);
        println!("Writer modified data to: {:?}", *values);
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}

fn threadpool_example() {
    println!("\n=== ThreadPool Example ===");
    
    // create a thread pool with 3 worker threads.
    let pool = ThreadPool::new(3);
    let shared_data = Arc::new(Mutex::new(vec![]));

    // Submit 5 jobs to the pool
    for i in 0..5 {
        let data_clone = Arc::clone(&shared_data);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(100));
            let mut data = data_clone.lock().unwrap();
            data.push(i);
            println!("Job {} completed by thread {:?}", i, thread::current().id());
        });
    }

    pool.join();
    println!("All jobs completed. Final data: {:?}", *shared_data.lock().unwrap());
}

pub fn demo() {
    basic_threads_example();
    channel_example();

    shared_state_example();
    rwlock_example();
    threadpool_example();
}