use std::{
    alloc::Layout,
    mem, ptr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::helper::{allocate_buffer, RingBufConsumer, RingBufProducer};

#[repr(C)]
pub struct Buffer<T> {
    buffer: *mut T,
    capacity: usize,
    allocated_size: usize,
    write_idx: AtomicUsize,
    read_idx: AtomicUsize,
}
unsafe impl<T: Sync> Sync for Buffer<T> {}

pub struct Consumer<T> {
    buffer: Arc<Buffer<T>>,
}

pub struct Producer<T> {
    buffer: Arc<Buffer<T>>,
}

unsafe impl<T: Send> Send for Consumer<T> {}
unsafe impl<T: Send> Send for Producer<T> {}

impl<T> Buffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = unsafe { allocate_buffer(capacity) };
        Self {
            buffer: ptr,
            capacity,
            allocated_size: capacity.next_power_of_two(),
            write_idx: AtomicUsize::new(0),
            read_idx: AtomicUsize::new(0),
        }
    }

    #[inline]
    fn buf_offset(&self, idx: usize) -> usize {
        idx & (self.allocated_size - 1)
    }

    #[inline]
    unsafe fn load(&self, pos: usize) -> T {
        let end = self.buffer.add(self.buf_offset(pos));
        ptr::read(end)
    }

    #[inline]
    unsafe fn store(&self, pos: usize, v: T) {
        let end = self.buffer.add(self.buf_offset(pos));
        ptr::write(&mut *end, v);
    }

    pub fn enqueue(&self, item: T) -> bool {
        let write_idx = self.write_idx.load(Ordering::Relaxed);
        let read_idx = self.read_idx.load(Ordering::Acquire);
        if write_idx - read_idx == self.capacity {
            return false;
        }

        unsafe {
            self.store(write_idx, item);
        }
        self.write_idx
            .store(write_idx.wrapping_add(1), Ordering::Release);
        true
    }

    pub fn dequeue(&self) -> Option<T> {
        let read_idx = self.read_idx.load(Ordering::Relaxed);
        let write_idx = self.write_idx.load(Ordering::Acquire);
        if write_idx == read_idx {
            return None;
        }

        let v = unsafe { self.load(read_idx) };
        self.read_idx
            .store(read_idx.wrapping_add(1), Ordering::Release);
        Some(v)
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        unsafe {
            let layout = Layout::from_size_align(
                self.allocated_size * mem::size_of::<T>(),
                mem::align_of::<T>(),
            )
            .unwrap();
            std::alloc::dealloc(self.buffer as *mut u8, layout);
        }
    }
}

pub fn make<T>(capacity: usize) -> (Producer<T>, Consumer<T>) {
    let arc = Arc::new(Buffer::with_capacity(capacity));

    (
        Producer {
            buffer: arc.clone(),
        },
        Consumer { buffer: arc },
    )
}

impl<T> RingBufProducer<T> for Producer<T> {
    fn enqueue(&self, item: T) -> bool {
        (*self.buffer).enqueue(item)
    }
}

impl<T> RingBufConsumer<T> for Consumer<T> {
    fn dequeue(&self) -> Option<T> {
        (*self.buffer).dequeue()
    }
}
