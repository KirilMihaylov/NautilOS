use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Default)]
pub struct CoreMutex<T> {
    lock: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> CoreMutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Lock<T> {
        while self
            .lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {}

        Lock::new(self)
    }
}

unsafe impl<T> Send for CoreMutex<T> where T: Send {}

unsafe impl<T> Sync for CoreMutex<T> where T: Send {}

pub struct Lock<'a, T> {
    mutex: &'a CoreMutex<T>,
    _phantom_data: PhantomData<&'a mut T>,
}

impl<'a, T> Lock<'a, T> {
    const fn new(mutex: &'a CoreMutex<T>) -> Self {
        Self {
            mutex,
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Drop for Lock<'_, T> {
    #[track_caller]
    fn drop(&mut self) {
        if let Err(false) =
            self.mutex
                .lock
                .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        {
            panic!("Internal error occured while unlocking CoreMutex! Mutex already unlocked!");
        }
    }
}

impl<T> Deref for Lock<'_, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        unsafe { &*(self.mutex.value.get()) }
    }
}

impl<T> DerefMut for Lock<'_, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        unsafe { &mut *(self.mutex.value.get()) }
    }
}
