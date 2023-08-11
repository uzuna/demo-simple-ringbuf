use std::{
    alloc::Layout,
    mem, ptr,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::helper::{allocate_buffer, RingBufTrait};

pub struct Buffer {
    buffer: *mut usize,
    capacity: usize,
    allocated_size: usize,
    write_idx: AtomicUsize,
    read_idx: AtomicUsize,
}

impl Buffer {
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
    unsafe fn load(&self, pos: usize) -> usize {
        let end = self.buffer.add(self.buf_offset(pos));
        ptr::read(end)
    }

    #[inline]
    unsafe fn store(&self, pos: usize, v: usize) {
        let end = self.buffer.add(self.buf_offset(pos));
        ptr::write(&mut *end, v);
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(
                self.allocated_size * mem::size_of::<usize>(),
                mem::align_of::<usize>(),
            )
            .unwrap();
            std::alloc::dealloc(self.buffer as *mut u8, layout);
        }
    }
}

impl RingBufTrait<usize> for Buffer {
    fn enqueue(&mut self, item: usize) -> bool {
        let write_idx = self.write_idx.load(Ordering::Relaxed);
        let read_idx = self.read_idx.load(Ordering::Acquire);
        if read_idx + self.capacity <= write_idx {
            return false;
        }

        unsafe {
            self.store(write_idx, item);
        }
        self.write_idx
            .store(write_idx.wrapping_add(1), Ordering::Release);
        true
    }

    fn dequeue(&mut self) -> Option<usize> {
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
