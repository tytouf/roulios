
pub mod exception;
pub mod systick;
pub mod scb;
pub mod mem;

mod lang_items;

#[no_mangle]
pub unsafe extern fn reset_handler() {
    mem::init_mem();

    ::start();
}

#[no_mangle]
pub unsafe extern fn abort() {
    asm!("bkpt");
    loop { }
}

pub fn wait_for_event() {
    unsafe { asm!("wfe"); }
}

pub fn wait_for_interrupt() {
    unsafe { asm!("wfi"); }
}

pub fn nop() {
    unsafe { asm!("nop"); }
}

pub fn enable_interrupts() {
    unsafe { asm!("cpsie i"); }
}

pub fn disable_interrupts() {
    unsafe { asm!("cpsid i"); }
}

pub fn set_psp(sp: u32) {
    unsafe { asm!("msr psp, $0" :: "r"(sp)); }
}
