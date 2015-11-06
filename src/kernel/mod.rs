
use alloc::boxed::Box;
use collections::Vec;

pub mod serial;
pub mod alloc;
pub mod sched;

pub struct KernelState {
    ticks: u32,
    tasks: sched::TaskList,
}

#[no_mangle] // this global var is used by asm
pub static mut kernel_state: *mut KernelState = 0 as *mut KernelState;

#[no_mangle] // this function is called by asm
pub fn kernel_tick(state: &mut KernelState) {
    state.ticks += 1;
}

pub fn init() {
    let state = Box::new(KernelState { ticks: 0xb00b, tasks: Vec::new()});
    unsafe {
        // Get raw pointer wrapped by the Box. Calling into_raw makes the
        // caller responsible for releasing the memory. In the case of the
        // global kernel state, the memory will never be released.
        kernel_state = Box::into_raw(state);
    }
}
