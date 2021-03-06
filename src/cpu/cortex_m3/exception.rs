use core::option::Option::{Some, self};
use cpu::cortex_m3::{reset_handler, abort, hard_fault, mem_mgt, bus_fault, usage_fault};

extern "C" {
    fn __STACK_TOP__();
    pub fn systick_handler();
    pub fn pendsv_handler();
    pub fn svc_handler();
}

/// Exception "vector"
#[link_section=".exception_vector"]
pub static VECTOR: [Option<unsafe extern fn()>; 16] = [
    Some(__STACK_TOP__),    // 0
    Some(reset_handler),    // 1
    Some(abort),            // 2
    Some(hard_fault),       // 3
    Some(mem_mgt),          // 4
    Some(bus_fault),        // 5
    Some(usage_fault),      // 6
    None,                   // 7
    None,                   // 8
    None,                   // 9
    None,                   // 10
    Some(svc_handler),      // 11
    Some(abort),            // 12
    None,                   // 13
    Some(pendsv_handler),   // 14
    Some(systick_handler),  // 15
];
