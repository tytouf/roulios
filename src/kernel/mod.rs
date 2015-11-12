
use alloc::boxed::Box;

pub mod serial;
pub mod alloc;
pub mod sched;

pub struct KernelState {
    ticks: u32,
    pub tasks: sched::TaskList,
}

#[no_mangle] // this global var is used by asm
pub static mut kernel_state: *mut KernelState = 0 as *mut KernelState;

#[no_mangle] // this function is called by asm
pub fn handle_exception(sp: u32, state: &mut KernelState) -> usize {
    {
        let cur_task = state.tasks.get_current_task_mut();
        cur_task.stack_ptr = sp as usize;
    }

    state.ticks += 1;

    let next_task = state.tasks.reschedule();
    return next_task.stack_ptr;
}

#[no_mangle] // this function is called by asm
pub fn handle_syscall(sp: u32, state: &mut KernelState) -> usize {
    {
        let cur_task = state.tasks.get_current_task_mut();
        cur_task.stack_ptr = sp as usize;
    }

    /* For now only one syscall that is equivalent to yield()
     */

    let next_task = state.tasks.reschedule();
    return next_task.stack_ptr;
}

pub fn init() -> &'static mut KernelState {
    let state = Box::new(
            KernelState { ticks: 0x0, tasks: sched::TaskList::new() });
    unsafe {
        // Get raw pointer wrapped by the Box. Calling into_raw makes the
        // caller responsible for releasing the memory. In the case of the
        // global kernel state, the memory will never be released.
        kernel_state = Box::into_raw(state);

        kernel_state.as_mut().unwrap()
    }
}
