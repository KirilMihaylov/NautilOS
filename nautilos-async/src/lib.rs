#![no_std]
#![feature(once_cell)]

use core::{
    future::Future,
    lazy::Lazy,
    mem::MaybeUninit,
    pin::Pin,
    ptr::{null, read, write},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

pub async fn yield_now() {
    struct Yield {
        ready: bool,
    }

    impl Future for Yield {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
            if self.ready {
                Poll::Ready(())
            } else {
                self.ready = true;

                context.waker().wake_by_ref();

                Poll::Pending
            }
        }
    }

    Yield { ready: false }.await
}

const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |data: *const ()| RawWaker::new(data, &RAW_WAKER_VTABLE),
    |_| (),
    |_| (),
    |_| (),
);

const RAW_WAKER: RawWaker = RawWaker::new(null(), &RAW_WAKER_VTABLE);

const WAKER: Lazy<Waker> = Lazy::new(|| unsafe { Waker::from_raw(RAW_WAKER) });

pub fn block_on<T>(future: T) -> T::Output
where
    T: Future,
{
    block_on_with(&mut Context::from_waker(&WAKER), future)
}

pub fn block_on_with<T>(context: &mut Context<'_>, mut future: T) -> T::Output
where
    T: Future,
{
    loop {
        let pinned: Pin<&mut T> = unsafe { Pin::new_unchecked(&mut future) };

        if let Poll::Ready(result) = pinned.poll(context) {
            break result;
        }
    }
}

pub fn block_on_array<T, const N: usize>(futures: [&mut dyn Future<Output = T>; N]) -> [T; N] {
    block_on_array_with(&mut Context::from_waker(&WAKER), futures)
}

pub fn block_on_array_with<T, const N: usize>(
    context: &mut Context<'_>,
    futures: [&mut dyn Future<Output = T>; N],
) -> [T; N] {
    let mut results: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };

    {
        let mut finished: [bool; N] = [false; N];

        while finished.iter().any(|finished: &bool| !finished) {
            for index in 0..N {
                if !finished[index] {
                    let pinned: Pin<&mut dyn Future<Output = T>> =
                        unsafe { Pin::new_unchecked(&mut *futures[index]) };

                    if let Poll::Ready(result) = pinned.poll(context) {
                        results[index] = result;

                        finished[index] = true;
                    }
                }
            }
        }
    }

    results
}

pub struct AwaitOnArray<'a, T, const N: usize> {
    futures: [&'a mut dyn Future<Output = T>; N],
    finished: [bool; N],
    results: [T; N],
}

impl<'a, T, const N: usize> AwaitOnArray<'a, T, N> {
    pub fn new(futures: [&'a mut dyn Future<Output = T>; N]) -> Self {
        Self {
            futures,
            finished: [false; N],
            results: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

impl<T, const N: usize> Future for AwaitOnArray<'_, T, N> {
    type Output = [T; N];

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        {
            while self.finished.iter().any(|finished: &bool| !finished) {
                for index in 0..N {
                    if !self.finished[index] {
                        let pinned: Pin<&mut dyn Future<Output = T>> = unsafe {
                            Pin::new_unchecked(
                                &mut *(&*self.futures[index] as *const dyn Future<Output = T>
                                    as *mut dyn Future<Output = T>),
                            )
                        };

                        if let Poll::Ready(result) = pinned.poll(context) {
                            unsafe {
                                write(&self.results[index] as *const _ as *mut _, result);

                                write(&self.finished[index] as *const _ as *mut _, true);
                            }
                        }
                    }
                }
            }
        }

        Poll::Ready(unsafe { read(&self.results) })
    }
}

#[macro_export]
macro_rules! block_on {
    ($future: expr $(,)?) => {
        $crate::block_on($future)
    };
    ($future: expr, $($futures: expr),+ $(,)?) => {
        $crate::block_on_array([&mut future, $(&mut $futures),+])
    };
}

#[macro_export]
macro_rules! await_on {
    ($future: expr $(,)?) => {
        $future.await
    };
    ($future: expr, $($futures: expr),+ $(,)?) => {
        $crate::AwaitOnArray::new([&mut $future, $(&mut $futures),+]).await
    };
}
