#![no_std]
#![feature(
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    maybe_uninit_ref
)]

use core::{
    future::Future,
    marker::PhantomPinned,
    mem::MaybeUninit,
    pin::Pin,
    ptr::{null, read},
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

const NEW_WAKER: fn() -> Waker = || {
    const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        |data: *const ()| RawWaker::new(data, &RAW_WAKER_VTABLE),
        |_| (),
        |_| (),
        |_| (),
    );

    const RAW_WAKER: RawWaker = RawWaker::new(null(), &RAW_WAKER_VTABLE);

    unsafe { Waker::from_raw(RAW_WAKER) }
};

pub fn block_on<T>(future: T) -> T::Output
where
    T: Future,
{
    block_on_with(&mut Context::from_waker(&NEW_WAKER()), future)
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

pub fn block_on_array<T, const N: usize>(futures: [T; N]) -> [T::Output; N]
where
    T: Future,
{
    block_on_array_with(&mut Context::from_waker(&NEW_WAKER()), futures)
}

pub fn block_on_array_with<T, const N: usize>(
    context: &mut Context<'_>,
    mut futures: [T; N],
) -> [T::Output; N]
where
    T: Future,
{
    let mut results: [MaybeUninit<T::Output>; N] = MaybeUninit::uninit_array();

    let mut futures: [Pin<&mut T>; N] = unsafe {
        let mut pinned_futures: [Pin<&mut T>; N] = MaybeUninit::zeroed().assume_init();

        {
            let mut futures: &mut [T] = &mut futures;

            for future in pinned_futures.iter_mut() {
                let (left, right): (&mut [T], &mut [T]) = futures.split_at_mut(1);

                futures = right;

                *future = Pin::new_unchecked(&mut left[0]);
            }
        }

        pinned_futures
    };

    {
        let mut finished: [bool; N] = [false; N];

        while finished.iter().any(|finished: &bool| !finished) {
            for index in 0..N {
                if !finished[index] {
                    if let Poll::Ready(result) = futures[index].as_mut().poll(context) {
                        unsafe {
                            *results[index].assume_init_mut() = result;
                        }

                        finished[index] = true;
                    }
                }
            }
        }
    }

    unsafe { MaybeUninit::array_assume_init(results) }
}

pub fn block_on_array_dyn<T, const N: usize>(
    futures: [Pin<&mut dyn Future<Output = T>>; N],
) -> [T; N] {
    block_on_array_dyn_with(&mut Context::from_waker(&NEW_WAKER()), futures)
}

pub fn block_on_array_dyn_with<T, const N: usize>(
    context: &mut Context<'_>,
    mut futures: [Pin<&mut dyn Future<Output = T>>; N],
) -> [T; N] {
    let mut results: [MaybeUninit<T>; N] = MaybeUninit::uninit_array();

    {
        let mut finished: [bool; N] = [false; N];

        while finished.iter().any(|finished: &bool| !finished) {
            for index in 0..N {
                if !finished[index] {
                    if let Poll::Ready(result) = futures[index].as_mut().poll(context) {
                        unsafe {
                            *results[index].assume_init_mut() = result;
                        }

                        finished[index] = true;
                    }
                }
            }
        }
    }

    unsafe { MaybeUninit::array_assume_init(results) }
}

pub struct AwaitOnArray<'a, T, const N: usize> {
    futures: [Pin<&'a mut dyn Future<Output = T>>; N],
    finished: [bool; N],
    results: [MaybeUninit<T>; N],
    _pinned: PhantomPinned,
}

impl<'a, T, const N: usize> AwaitOnArray<'a, T, N> {
    pub fn new(futures: [Pin<&'a mut dyn Future<Output = T>>; N]) -> Self {
        Self {
            futures,
            finished: [false; N],
            results: MaybeUninit::uninit_array(),
            _pinned: PhantomPinned,
        }
    }
}

impl<T, const N: usize> Future for AwaitOnArray<'_, T, N> {
    type Output = [T; N];

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this: &mut Self = unsafe { self.get_unchecked_mut() };

        while this.finished.iter().any(|finished: &bool| !finished) {
            for index in 0..N {
                if !this.finished[index] {
                    if let Poll::Ready(result) = this.futures[index].as_mut().poll(context) {
                        unsafe {
                            *this.results[index].assume_init_mut() = result;
                        }

                        this.finished[index] = true;

                        let _ = unsafe { Pin::new_unchecked(&mut yield_now()) }.poll(context);
                    }
                }
            }
        }

        Poll::Ready(unsafe { MaybeUninit::array_assume_init(read(&this.results)) })
    }
}

#[macro_export]
macro_rules! block_on {
    ($future: expr $(,)?) => {{
        let mut future: _ = $future;

        $crate::block_on(unsafe { core::pin::Pin::new_unchecked(&mut future) })
    }};
    ($future: expr, $($futures: expr),+ $(,)?) => {{
        let (mut $future_ident, $(mut $future_idents),+): _ = ($future, $($futures),+);

        $crate::block_on_array(unsafe { [core::pin::Pin::new_unchecked(&mut future), $(core::pin::Pin::new_unchecked(&mut $futures)),+] })
    }};
}

#[macro_export]
macro_rules! await_on {
    ($future_ident: ident = $future: expr, $($future_idents: ident = $futures: expr),+ $(,)?) => {{
        let (mut $future_ident, $(mut $future_idents),+): _ = ($future, $($futures),+);

        $crate::AwaitOnArray::new(unsafe { [core::pin::Pin::new_unchecked(&mut $future_ident), $(core::pin::Pin::new_unchecked(&mut $future_idents)),+] }).await
    }};
}
