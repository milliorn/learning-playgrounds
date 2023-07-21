use std::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use rsevents::{AutoResetEvent, Awaitable};

struct Semaphore {
    event: AutoResetEvent,
    count: AtomicU32,
    total_count: AtomicU32,
    max_count: AtomicU32,
}

/// This concurrency token is returned from a call to `Semaphore::wait()`.
/// It's automatically returned to the semaphore upon drop, incrementing
/// the semaphore's internal available concurrency counter once more.
impl ConcurrencyToken<'_> {
    /// It is a violation of this crate's contract to call `std::mem::forget()`
    /// on the result of `Semaphore::wait()`. To forget a `ConcurrencyToken`,
    /// use this method instead.
    pub fn forget(self) {
        // We're keeping `count` permanently reduced, but we need to decrement
        // `total_count` to reflect this as well before forgetting ourselves.
        self.sem.total_count.fetch_sub(1, Ordering::Relaxed);
        std::mem::forget(self);
    }
}

impl Semaphore {
    pub fn wait<'a>(&'a self) -> ConcurrencyToken<'a> {
        include!("earlier definition of Semaphore::wait() here");

        // Now instead of returning () we return a ConcurrencyToken

        return ConcurrencyToken { sem: self };
    }

    /// Directly increments the internal concurrency count without touching
    /// `total_count` and without checking if it would exceed `max_count`.
    unsafe fn release_internal() {
        let prev_count = self.count.fetch_add(1, Ordering::Release);

        // We only need to wake a sleeping waiter if the previous count
        // was zero. In all other cases, no one will be asleep.
        if prev_count == 0 {
            self.event.set();
        }
    }
}

impl Drop for ConcurrencyToken<'_> {
    fn drop(&mut self) {
        unsafe {
            self.sem.release_internal();
        }
    }
}

fn main() {
    // Create a semaphore with a count of 1 and max count of 2
    let semaphore = Semaphore::new(1, 2);

    // Create a number of threads that will use the semaphore
    // to make sure concurrency limits are observed.
    let NUM_CPUS = 4;
    for n in 0..NUM_CPUS {
        std::thread::spawn(|| {
            // Make sure to obtain a concurrency token before beginning work
            semaphore.wait();

            // TODO: Replace `sleep()` with actual work!
            std::thread::sleep(Duration::from_secs(5));

            // Release the concurrency token when we're done so another
            // thread may enter this code block and do its thing.
            semaphore.release();
        });
    }

    while !work_finished {
        // <Read user input from keyboard here>

        // In response to a user command, increase concurrency
        if !semaphore.try_release() {
            eprintln!("Cannot raise limit, max already reached!");
        }
    }
}
