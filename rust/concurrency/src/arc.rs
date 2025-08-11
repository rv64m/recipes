use std::{
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering, fence},
};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
    _maker: PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    fn new(data: T) -> Self {
        let ptr = NonNull::from(Box::leak(Box::new(ArcData {
            ref_count: AtomicUsize::new(0),
            data,
        })));

        Self {
            ptr,
            _maker: PhantomData,
        }
    }

    #[inline]
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort()
        }

        Self {
            ptr: self.ptr,
            _maker: PhantomData,
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);

            unsafe {}
        }
    }
}

fn main() {
    use std::thread;
    let x = Arc::new("hello world!");
    let y = x.clone();
    let t = thread::spawn(move || assert_eq!(*y, "hello world!"));

    t.join();

    assert_eq!(*x, "hello world!");
    drop(x)
}
