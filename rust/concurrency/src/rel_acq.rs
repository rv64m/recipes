use std::{
    sync::{
        Arc,
        atomic::{self, AtomicBool, AtomicI32},
    },
    thread,
    time::Duration,
};

fn main() {
    let x = Arc::new(AtomicI32::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let x_clone = Arc::clone(&x);
    let ready_clone = Arc::clone(&ready);
    thread::spawn(move || {
        x_clone.store(1, atomic::Ordering::Relaxed);
        ready_clone.store(true, atomic::Ordering::Release);
    });

    while !ready.load(atomic::Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    assert!(x.load(atomic::Ordering::Relaxed) == 1);
}
