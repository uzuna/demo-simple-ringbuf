use std::{alloc::Layout, mem, ptr};

use crate::helper::{allocate_buffer, RingBufTrait};

/// Ringbuffer
/// index calculation: by and
/// don't support multi-threding
#[derive(Debug)]
pub struct RingBuf<T> {
    buf: *mut T,
    capacity: usize,
    position_mask: usize,
    read_idx: usize,
    write_idx: usize,
}

impl<T> RingBuf<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = unsafe { allocate_buffer(capacity) };
        Self {
            buf: ptr,
            capacity,
            position_mask: capacity.next_power_of_two() - 1,
            read_idx: 0,
            write_idx: 0,
        }
    }

    #[inline]
    fn to_ptr(&self, pos: usize) -> usize {
        pos & (self.position_mask)
    }

    #[inline]
    unsafe fn load(&self, pos: usize) -> T {
        let ptr = self.buf.add(pos);
        ptr::read(ptr)
    }

    #[inline]
    unsafe fn store(&self, pos: usize, v: T) {
        let ptr = self.buf.add(pos);
        ptr::write(&mut *ptr, v);
    }
}

impl<T> RingBufTrait<T> for RingBuf<T> {
    fn enqueue(&mut self, item: T) -> bool {
        if (self.write_idx - self.read_idx) == self.capacity {
            return false;
        }
        unsafe {
            self.store(self.to_ptr(self.write_idx), item);
        }
        self.write_idx += 1;
        true
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.read_idx == self.write_idx {
            return None;
        }
        let item = unsafe { self.load(self.to_ptr(self.read_idx)) };
        self.read_idx += 1;
        Some(item)
    }
}

impl<T> Drop for RingBuf<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        unsafe {
            let layout = Layout::from_size_align(
                (self.position_mask + 1) * mem::size_of::<T>(),
                mem::align_of::<T>(),
            )
            .unwrap();
            std::alloc::dealloc(self.buf as *mut u8, layout);
        }
    }
}
