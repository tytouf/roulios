/// Initialize memory by copying data from ROM to RAM and zeroing the bss
/// region.
pub fn init_mem() {
    extern {
        /* Variables expected from the linker script
        */
        static __data_load: u32;
        static mut __data_start: u32;
        static mut __data_end: u32;
        static mut __bss_start: u32;
        static mut __bss_end: u32;
    }

    unsafe {
        // __data_end and __bss_end are aligned on 4 bytes in linker script.

        // Start by copying data from ROM to RAM.
        let mut load_addr: *const u32 = &__data_load;
        let mut mem_addr: *mut u32 = &mut __data_start;
        while mem_addr < &mut __data_end as *mut u32 {
            *mem_addr = *load_addr;
            mem_addr = ((mem_addr as u32) + 4) as *mut u32;
            load_addr = ((load_addr as u32) + 4) as *const u32;
        }

        // Then clear BSS in RAM
        mem_addr = &mut __bss_start;
        while mem_addr < &mut __bss_end as *mut u32 {
            *mem_addr = 0u32;
            mem_addr = ((mem_addr as u32) + 4) as *mut u32;
        }
    }
}
