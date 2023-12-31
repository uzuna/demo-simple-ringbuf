use std::{
    alloc::Layout,
    cell::Cell,
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
    position_mask: usize,
    // PCで別のスレッドが触るので個別のL1キャッシュに乗るようにPaddingで埋めて分割する
    _padding0: [usize; crate::cacheline_pad!(3)],
    write_idx: AtomicUsize,
    cached_read_idx: Cell<usize>,
    // cached_read_count: Cell<usize>,
    _padding1: [usize; crate::cacheline_pad!(2)],
    read_idx: AtomicUsize,
    cached_write_idx: Cell<usize>,
    // cached_write_count: Cell<usize>,
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
            position_mask: capacity.next_power_of_two() - 1,
            _padding0: [0; crate::cacheline_pad!(3)],
            write_idx: AtomicUsize::new(0),
            cached_read_idx: Cell::new(0),
            // cached_read_count: Cell::new(0),
            _padding1: [0; crate::cacheline_pad!(2)],
            read_idx: AtomicUsize::new(0),
            cached_write_idx: Cell::new(0),
            // cached_write_count: Cell::new(0),
        }
    }

    #[inline]
    fn buf_offset(&self, idx: usize) -> usize {
        idx & (self.position_mask)
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
        if self.cached_read_idx.get() + self.capacity <= write_idx {
            self.cached_read_idx
                .set(self.read_idx.load(Ordering::Acquire));
            assert!(self.cached_read_idx.get() <= write_idx);
            // println!("enqueue: w,r: {},{}", write_idx, self.cached_read_idx.get());
            // self.cached_read_count.set(self.cached_read_count.get() + 1);
            if write_idx - self.cached_read_idx.get() == self.capacity {
                return false;
            }
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
        if self.cached_write_idx.get() == read_idx {
            self.cached_write_idx
                .set(self.write_idx.load(Ordering::Acquire));
            assert!(read_idx <= self.cached_write_idx.get());
            // println!("dequeue: w,r: {},{}", self.cached_write_idx.get(), read_idx);
            // self.cached_write_count
            //     .set(self.cached_write_count.get() + 1);
            if self.cached_write_idx.get() == read_idx {
                return None;
            }
        }

        let v = unsafe { self.load(read_idx) };
        self.read_idx
            .store(read_idx.wrapping_add(1), Ordering::Release);
        Some(v)
    }

    // pub fn show_cache(&self) {
    //     println!(
    //         "read: {}, write: {}",
    //         self.cached_read_count.get(),
    //         self.cached_write_count.get()
    //     );
    // }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        unsafe {
            let layout = Layout::from_size_align(
                (self.position_mask + 1) * mem::size_of::<T>(),
                mem::align_of::<T>(),
            )
            .unwrap();
            std::alloc::dealloc(self.buffer as *mut u8, layout);
        }
    }
}

pub fn make<T>(capacity: usize) -> (Producer<T>, Consumer<T>, Arc<Buffer<T>>) {
    let arc = Arc::new(Buffer::with_capacity(capacity));

    (
        Producer {
            buffer: arc.clone(),
        },
        Consumer {
            buffer: arc.clone(),
        },
        arc,
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
