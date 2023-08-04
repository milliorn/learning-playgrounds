// Enhanced Hashing Pipeline with Statistics

// Import necessary crates
use sha2::{Digest, Sha512};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

// Define the number of hashes to compute
const N: usize = 100000; // 1_000_000_000

// Define the number of sha512 and blake3 hashers to use
const NUM_BLAKE3_HASHERS: usize = 4;
const NUM_SHA512_HASHERS: usize = 8;

fn main() {
    // Start the timer
    let start = Instant::now();

    // Create ring buffers for communication between threads
    let (mut generator_to_sha512_tx, mut generator_to_sha512_rx) =
        ring_buffers(NUM_SHA512_HASHERS, 1_000_000);
    let (mut generator_to_blake3_tx, mut generator_to_blake3_rx) =
        ring_buffers(NUM_BLAKE3_HASHERS, 1_000_000);
    let (mut sha512_to_result_tx, mut sha512_to_result_rx) =
        ring_buffers(NUM_SHA512_HASHERS, 1_000_000);
    let (mut blake3_to_result_tx, mut blake3_to_result_rx) =
        ring_buffers(NUM_BLAKE3_HASHERS, 1_000_000);

    // Create a vector to store statistics (idle and blocked times) for each stage
    let mut stats = vec![];

    // Generator thread to generate preimages
    let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
    stats.push((format!("generator"), idle.clone(), blocked.clone()));
    thread::spawn(move || {
        let mut sha512_channel = 0; // Round robin to sha512 threads
        let mut blake3_channel = 0; // Round robin to blake3 threads

        for i in 0..N {
            let preimage = (i as u64).to_le_bytes(); // Convert i to a little endian byte array
            push(
                &mut generator_to_sha512_tx[sha512_channel], // Push to sha512 thread
                preimage.clone(),                            // Clone the preimage
                &blocked,
            );
            push(
                &mut generator_to_blake3_tx[blake3_channel], // Push to blake3 thread
                preimage,                                    // Move the preimage
                &blocked,
            );
            sha512_channel = (sha512_channel + 1) % NUM_SHA512_HASHERS; // Round robin
            blake3_channel = (blake3_channel + 1) % NUM_BLAKE3_HASHERS; // Round robin
        }
    });

    // Sha512 threads to perform the hashing
    for i in 0..NUM_SHA512_HASHERS {
        let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0))); // Create idle and blocked counters
        stats.push((format!("sha512_{}", i), idle.clone(), blocked.clone())); // Add to stats vector
        let mut rx = generator_to_sha512_rx.remove(0); // Get the rx channel from the generator
        let mut tx = sha512_to_result_tx.remove(0); // Get the tx channel to the result thread
        thread::spawn(move || loop {
            let preimage = pop(&mut rx, &idle); // Pop a preimage from the generator
            let hash = Sha512::digest(&preimage); // Hash the preimage
            push(&mut tx, hash, &blocked); // Push the hash to the result thread
        });
    }

    // Blake3 threads to perform the hashing
    for i in 0..NUM_BLAKE3_HASHERS {
        let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0))); // Create idle and blocked counters
        stats.push((format!("blake3_{}", i), idle.clone(), blocked.clone())); // Add to stats vector
        let mut rx = generator_to_blake3_rx.remove(0); // Get the rx channel from the generator
        let mut tx = blake3_to_result_tx.remove(0); // Get the tx channel to the result thread
        thread::spawn(move || loop {
            let preimage = pop(&mut rx, &idle); // Pop a preimage from the generator
            let hash = blake3::hash(&preimage); // Hash the preimage
            push(&mut tx, hash, &blocked); // Push the hash to the result thread
        });
    }

    // Result thread to receive hashes from sha512 and blake3 threads
    let (idle, blocked) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0))); // Create idle and blocked counters
    stats.push((format!("result"), idle.clone(), blocked.clone())); // Add to stats vector
    let result_thread = thread::spawn(move || {
        let mut sha512_channel = 0; // Round robin to sha512 threads
        let mut blake3_channel = 0; // Round robin to blake3 threads
        for _ in 0..N {
            pop(&mut sha512_to_result_rx[sha512_channel], &idle); // Pop from sha512 thread
            pop(&mut blake3_to_result_rx[blake3_channel], &idle); // Pop from blake3 thread
            sha512_channel = (sha512_channel + 1) % NUM_SHA512_HASHERS; // Round robin
            blake3_channel = (blake3_channel + 1) % NUM_BLAKE3_HASHERS; // Round robin
        }
    });

    // Stats thread to display idle and blocked percentages
    thread::spawn(move || {
        let stats_interval = Duration::from_secs(1); // Print stats every second
        let start = Instant::now(); // Start the timer
        loop {
            for (name, idle, blocked) in stats.iter() {
                let percent_idle = (100.0 * idle.load(Ordering::Relaxed) as f64
                    / start.elapsed().as_millis() as f64) as i32; // Calculate the percentage of time idle
                let percent_blocked = (100.0 * blocked.load(Ordering::Relaxed) as f64
                    / start.elapsed().as_millis() as f64)
                    as i32; // Calculate the percentage of time blocked
                println!(
                    "{}: %idle={} %blocked={}",
                    name, percent_idle, percent_blocked
                );
            }
            println!("");
            thread::sleep(stats_interval); // Sleep for the stats interval
        }
    });

    result_thread.join().unwrap(); // Wait for the result thread to finish

    // Print the time elapsed
    println!("{:?}", start.elapsed());
}

// Helper function to create ring buffers
fn ring_buffers<T>(
    count: usize,
    capacity: usize,
) -> (Vec<rtrb::Producer<T>>, Vec<rtrb::Consumer<T>>) {
    (0..count).map(|_| rtrb::RingBuffer::new(capacity)).unzip()
}

// Helper function to push data into the ring buffer with retry
fn push<T>(tx: &mut rtrb::Producer<T>, mut value: T, blocked: &Arc<AtomicU64>) {
    loop {
        match tx.push(value) {
            Ok(_) => break,                             // Push succeeded
            Err(rtrb::PushError::Full(v)) => value = v, // Ring buffer is full, retry with the same value
        }
        let start = Instant::now(); // Start the timer
        thread::sleep(Duration::from_millis(10));
        blocked.fetch_add(start.elapsed().as_millis() as u64, Ordering::Relaxed);
        // Increment the blocked counter
    }
}

// Helper function to pop data from the ring buffer with retry
fn pop<T>(rx: &mut rtrb::Consumer<T>, idle: &Arc<AtomicU64>) -> T {
    loop {
        if let Ok(value) = rx.pop() {
            return value; // Pop succeeded
        }
        let start = Instant::now(); // Start the timer
        thread::sleep(Duration::from_millis(10));
        idle.fetch_add(start.elapsed().as_millis() as u64, Ordering::Relaxed); // Increment the idle counter
    }
}
