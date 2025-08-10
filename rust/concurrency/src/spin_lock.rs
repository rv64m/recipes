use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;
use std::thread;

struct SpinLock<T: Send> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

struct SpinLockGuard<'a, T: Send> {
    lock: &'a SpinLock<T>,
}

impl<'a, T: Send> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release)
    }
}

impl<'a, T: Send> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T: Send> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> SpinLock<T>
where
    T: Send,
{
    pub fn new(data: T) -> SpinLock<T> {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock<'a>(&'a self) -> SpinLockGuard<'a, T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }

        SpinLockGuard { lock: self }
    }

    #[allow(unused)]
    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    let v = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            v.lock().push(1);
        });

        s.spawn(|| {
            v.lock().push(2);
        });
    });

    assert!(v.lock().len() == 2)
}
