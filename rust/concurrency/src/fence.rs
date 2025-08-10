use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};
use std::thread;

static DATA: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    // Writer thread
    let writer = thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        fence(Ordering::Release);
        READY.store(true, Ordering::Relaxed);
    });

    // Reader thread
    let reader = thread::spawn(|| {
        while !READY.load(Ordering::Relaxed) {}
        fence(Ordering::Acquire);
        let value = DATA.load(Ordering::Relaxed);
        assert_eq!(value, 42);
    });

    writer.join().unwrap();
    reader.join().unwrap();
}
