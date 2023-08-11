use std::{alloc::Layout, mem, ptr};

use crate::helper::{allocate_buffer, RingBufTrait};

/// Ringbuffer
/// index calculation: by modulo
/// don't support multi-threding
#[derive(Debug)]
pub struct RingBuf {
    buf: *mut usize,
    capacity: usize,
    allocated_size: usize,
    read_idx: usize,
    write_idx: usize,
}

impl RingBuf {
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
    fn to_ptr(&self, pos: usize) -> usize {
        pos % (self.allocated_size - 1)
    }

    #[inline]
    unsafe fn load(&self, pos: usize) -> usize {
        let ptr = self.buf.add(pos);
        ptr::read(ptr)
    }

    #[inline]
    unsafe fn store(&self, pos: usize, v: usize) {
        let ptr = self.buf.add(pos);
        ptr::write(&mut *ptr, v);
    }
}

impl RingBufTrait<usize> for RingBuf {
    fn enqueue(&mut self, item: usize) -> bool {
        if (self.write_idx - self.read_idx) == self.capacity {
            return false;
        }
        unsafe {
            self.store(self.to_ptr(self.write_idx), item);
        }
        self.write_idx += 1;
        true
    }

    fn dequeue(&mut self) -> Option<usize> {
        if self.read_idx == self.write_idx {
            return None;
        }
        let item = unsafe { self.load(self.to_ptr(self.read_idx)) };
        self.read_idx += 1;
        Some(item)
    }
}

impl Drop for RingBuf {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        unsafe {
            let layout = Layout::from_size_align(
                self.allocated_size * mem::size_of::<usize>(),
                mem::align_of::<usize>(),
            )
            .unwrap();
            std::alloc::dealloc(self.buf as *mut u8, layout);
        }
    }
}
