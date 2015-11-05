 /* Memory dynamic allocation from memory pool.
  * At the moment, only memory allocation is implemented for sake of simplicity.
  */
struct MemPoolAllocator {
    start_addr: usize,
    size: usize,
    next_free_slot: usize,
}

static mut mem_allocator: MemPoolAllocator = MemPoolAllocator {
    start_addr: 0,
    size: 0,
    next_free_slot: 0,
};

#[inline(never)]
pub fn init_allocator(start_addr: usize, size: usize) {
    unsafe {
        mem_allocator.start_addr = start_addr;
        mem_allocator.size = size;
        mem_allocator.next_free_slot = mem_allocator.start_addr;
    }
}

#[no_mangle]
pub unsafe fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    let mask = align - 1;
    let addr = (mem_allocator.next_free_slot + mask) & !mask;
    mem_allocator.next_free_slot = addr + size;
    return addr as *mut u8;
}

#[no_mangle]
pub fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    // Do nothing for this dummy allocator that can only allocate.
}

#[no_mangle]
pub unsafe fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                     align: usize) -> *mut u8 {
    // We don't de-allocate so just allocate a new mem
    __rust_allocate(size, align)
}
