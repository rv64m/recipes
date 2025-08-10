use std::sync::atomic::AtomicBool;
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);
static mut S: String = String::new();

#[allow(static_mut_refs)]
fn main() {
    let t1 = thread::spawn(|| {
        A.store(true, std::sync::atomic::Ordering::SeqCst);
        if B.load(std::sync::atomic::Ordering::SeqCst) {
            unsafe {
                S.push_str("A");
            }
        }
    });

    let t2 = thread::spawn(|| {
        B.store(true, std::sync::atomic::Ordering::SeqCst);
        if A.load(std::sync::atomic::Ordering::SeqCst) {
            unsafe {
                S.push_str("B");
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert!(unsafe { S != "AB" && S != "BA" });
    println!("{}", unsafe { S.as_str() });
}
