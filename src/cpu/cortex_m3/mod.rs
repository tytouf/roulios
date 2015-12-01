
pub mod exception;
pub mod systick;
pub mod scb;
pub mod mem;

mod lang_items;

#[no_mangle]
pub unsafe extern "C" fn reset_handler() {
    mem::init_mem();

    ::start();
}

#[no_mangle]
pub unsafe extern "C" fn abort() {
    asm!("bkpt");
    loop { }
}

#[no_mangle]
pub unsafe extern "C" fn bus_fault() {
    asm!("bkpt");
    loop { }
}

#[no_mangle]
pub unsafe extern "C" fn mem_mgt() {
    asm!("bkpt");
    loop { }
}

#[no_mangle]
pub unsafe extern "C" fn usage_fault() {
    asm!("bkpt");
    loop { }
}

#[no_mangle]
pub unsafe extern "C" fn hard_fault() {
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

#[inline(never)]
pub fn svc() {
    unsafe { asm!("svc 0"); }
}

#[macro_export]
macro_rules! svc {
    ($number:expr) => {
        unsafe { asm!(concat!("svc ", $number)); }
    }
}

pub fn enable_interrupts() {
    unsafe { asm!("cpsie i"); }
}

pub fn disable_interrupts() {
    unsafe { asm!("cpsid i"); }
}

#[inline(never)]
pub fn set_psp(sp: u32) {
    unsafe { asm!("msr psp, $0" :: "r"(sp)); }
}
