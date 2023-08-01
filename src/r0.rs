use std::{alloc::Layout, mem, ptr};

use crate::helper::allocate_buffer;

/// Ringbuffer 0
/// index calculation: by modulo
/// don't support multi-threding
#[derive(Debug)]
pub struct RingBuf0<T> {
    buf: *mut T,
    capacity: usize,
    allocated_size: usize,
    read_idx: usize,
    write_idx: usize,
}

impl<T> RingBuf0<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = unsafe { allocate_buffer(capacity) };
        Self {
            buf: ptr,
            capacity,
            allocated_size: capacity.next_power_of_two(),
            read_idx: 0,
            write_idx: 0,
        }
    }

    #[inline]
    fn read_ptr(&self) -> usize {
        self.read_idx & (self.allocated_size - 1)
    }
    #[inline]
    fn write_ptr(&self) -> usize {
        self.write_idx & (self.allocated_size - 1)
    }

    #[inline]
    unsafe fn load(&self, pos: usize) -> T {
        let end = self.buf.add(pos);
        ptr::read(end)
    }

    #[inline]
    unsafe fn store(&self, pos: usize, v: T) {
        let end = self.buf.add(pos);
        ptr::write(&mut *end, v);
    }

    pub fn enqueue(&mut self, item: T) -> bool {
        if (self.write_idx - self.read_idx) == self.capacity {
            return false;
        }
        unsafe {
            self.store(self.write_ptr(), item);
        }
        self.write_idx += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.read_idx == self.write_idx {
            return None;
        }
        let item = unsafe { self.load(self.read_ptr()) };
        self.read_idx += 1;
        Some(item)
    }
}

impl<T> Drop for RingBuf0<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        unsafe {
            let layout = Layout::from_size_align(
                self.allocated_size * mem::size_of::<T>(),
                mem::align_of::<T>(),
            )
            .unwrap();
            std::alloc::dealloc(self.buf as *mut u8, layout);
        }
    }
}
