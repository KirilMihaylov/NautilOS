mod core_mutex;

extern crate alloc;

use {
    crate::{
        heap::{init_in_place::initialization_error::InitializationError, Heap},
        heap_error::HeapError,
    },
    core::{
        alloc::{GlobalAlloc, Layout},
        ptr::{null_mut, NonNull},
    },
    alloc::alloc::handle_alloc_error,
    core_mutex::{CoreMutex, Lock},
};

#[global_allocator]
static ALLOC_IMPL: Allocator = Allocator::new();

struct HeapState {
    poisoned: bool,
    heap: &'static mut Heap,
}

impl HeapState {
    pub const fn new(heap: &'static mut Heap) -> Self {
        Self {
            poisoned: false,
            heap,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AllocatorInitializationError {
    DoubleInitialization,

    InitializationError(InitializationError),
}

struct Allocator {
    mutex: CoreMutex<Option<HeapState>>,
}

impl Allocator {
    const fn new() -> Self {
        Self {
            mutex: CoreMutex::new(None),
        }
    }

    fn initialize(&self, memory: &'static mut [u8]) -> Result<(), AllocatorInitializationError> {
        let mut lock: Lock<Option<HeapState>> = self.mutex.lock();

        if lock.is_some() {
            return Err(AllocatorInitializationError::DoubleInitialization);
        }

        *lock = Some(HeapState::new(
            Heap::init_in_place(memory)
                .map_err(AllocatorInitializationError::InitializationError)?,
        ));

        Ok(())
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut lock: Lock<Option<HeapState>> = self.mutex.lock();

        match &mut *lock {
            Some(state) => {
                if state.poisoned {
                    return null_mut();
                }

                let result: Result<NonNull<u8>, HeapError> = state.heap.alloc_from_layout(
                    if let Ok(layout) = Layout::from_size_align(layout.size(), layout.align()) {
                        layout
                    } else {
                        return null_mut();
                    },
                );

                match result {
                    Ok(pointer) => pointer.as_ptr(),
                    Err(error) => {
                        if let HeapError::InternalError = error {
                            state.poisoned = true;

                            handle_alloc_error(layout);
                        }

                        null_mut()
                    }
                }
            }
            None => null_mut(),
        }
    }

    unsafe fn realloc(&self, address: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let mut lock: Lock<Option<HeapState>> = self.mutex.lock();

        match &mut *lock {
            Some(state) => {
                let result: Result<NonNull<u8>, HeapError> = state.heap.realloc_from_layout(
                    address,
                    if let Ok(layout) = Layout::from_size_align(layout.size(), layout.align()) {
                        layout
                    } else {
                        return null_mut();
                    },
                    if let Ok(layout) = Layout::from_size_align(new_size, layout.align()) {
                        layout
                    } else {
                        return null_mut();
                    },
                );

                match result {
                    Ok(pointer) => pointer.as_ptr(),
                    Err(error) => {
                        if let HeapError::InternalError = error {
                            state.poisoned = true;

                            handle_alloc_error(layout);
                        }

                        null_mut()
                    }
                }
            }
            None => null_mut(),
        }
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        let mut lock: Lock<Option<HeapState>> = self.mutex.lock();

        match &mut *lock {
            Some(state) => {
                let result: Result<(), HeapError> = state.heap.dealloc_from_layout(
                    address,
                    if let Ok(layout) = Layout::from_size_align(layout.size(), layout.align()) {
                        layout
                    } else {
                        return;
                    },
                );

                if let Err(HeapError::InternalError) = result {
                    state.poisoned = true;

                    handle_alloc_error(layout);
                }
            }
            None => (),
        }
    }
}

unsafe impl Send for Allocator {}

unsafe impl Sync for Allocator {}

#[cfg_attr(not(test), alloc_error_handler)]
#[cfg_attr(test, allow(dead_code))]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!(
        "Error occured while allocating memory!\nLayout passed as an argument: {:?}",
        layout
    );
}

/// # Errors
/// TODO
pub fn initialize(memory: &'static mut [u8]) -> Result<(), AllocatorInitializationError> {
    ALLOC_IMPL.initialize(memory)
}
