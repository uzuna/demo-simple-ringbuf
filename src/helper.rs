pub unsafe fn allocate_buffer<T>(capacity: usize) -> *mut T {
    let adjusted_size = capacity.next_power_of_two();
    let layout = std::alloc::Layout::array::<T>(adjusted_size).unwrap();
    let ptr = std::alloc::alloc(layout);
    if ptr.is_null() {
        panic!("failed to allocate memory");
    }
    ptr as *mut T
}

pub trait RingBufTrait<T> {
    fn enqueue(&mut self, item: T) -> bool;
    fn dequeue(&mut self) -> Option<T>;
}

pub trait RingBufProducer<T> {
    fn enqueue(&self, item: T) -> bool;
}
pub trait RingBufConsumer<T> {
    fn dequeue(&self) -> Option<T>;
}
