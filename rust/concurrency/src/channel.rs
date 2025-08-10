mod oneshot {
    use std::{
        cell::UnsafeCell,
        marker::PhantomData,
        mem::MaybeUninit,
        sync::atomic::{AtomicU8, Ordering},
        thread::{self, Thread},
    };

    const EMPTY: u8 = 0;
    const READING: u8 = 1;
    const WRITING: u8 = 2;
    const READY: u8 = 3;
    pub struct Sender<'a, T> {
        ch: &'a Channel<T>,
        current_thread: Thread,
    }

    impl<'a, T> Sender<'a, T> {
        pub fn send(self, message: T) {
            if self
                .ch
                .ready
                .compare_exchange(EMPTY, WRITING, Ordering::Relaxed, Ordering::Relaxed)
                .is_err()
            {
                panic!("no messages to send one")
            }
            unsafe {
                (*self.ch.message.get()).write(message);
            }
            self.ch.ready.store(READY, Ordering::Release);
            self.current_thread.unpark();
        }
    }

    pub struct Receiver<'a, T> {
        ch: &'a Channel<T>,
        _no_send: PhantomData<*const ()>,
    }

    impl<'a, T> Receiver<'a, T> {
        pub fn is_ready(&self) -> bool {
            self.ch.ready.load(Ordering::Relaxed) == READY
        }

        pub fn receive(self) -> T {
            while !self.is_ready() {
                thread::park();
            }

            if self
                .ch
                .ready
                .compare_exchange(READY, READING, Ordering::Acquire, Ordering::Relaxed)
                .is_err()
            {
                panic!("no message available!")
            }

            unsafe { (*self.ch.message.get()).assume_init_read() }
        }
    }

    pub struct Channel<T> {
        ready: AtomicU8,
        message: UnsafeCell<MaybeUninit<T>>,
    }

    unsafe impl<T> Sync for Channel<T> where T: Send {}

    impl<T> Channel<T> {
        pub fn new() -> Self {
            Self {
                ready: AtomicU8::new(EMPTY),
                message: UnsafeCell::new(MaybeUninit::uninit()),
            }
        }

        pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
            *self = Self::new();
            (
                Sender {
                    ch: self,
                    current_thread: thread::current(),
                },
                Receiver {
                    ch: self,
                    _no_send: PhantomData,
                },
            )
        }
    }

    impl<T> Drop for Channel<T> {
        fn drop(&mut self) {
            if *self.ready.get_mut() == READY {
                unsafe {
                    self.message.get_mut().assume_init_drop();
                }
            }
        }
    }
}

fn main() {
    use crate::oneshot::Channel;
    use std::thread;
    use std::{thread::sleep, time::Duration};

    let mut ch = Channel::new();
    thread::scope(|s| {
        let (tx, rx) = ch.split();
        s.spawn(move || {
            tx.send("hello world!");
            sleep(Duration::from_secs(5));
        });

        assert!(rx.receive() == "hello world!")
    })
}
