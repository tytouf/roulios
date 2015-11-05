/*
 * System Control Block
 *
 * Cortex-m3 Reference Manual, Chapter 8.2.2 - CPU ID Base registers, etc.
 *
 */

use volatile::{RW, RO};

#[repr(C)]
pub struct SystemControlBlockDevice {
    /* 0x00 */ pub cpuid: RO<u32>,
    /* 0x04 */ pub icsr: RW<u32>,
}

pub struct SystemControlBlock {
    device: &'static SystemControlBlockDevice,
}

pub struct CpuId {
    part_no: u16,
    implementer: u8,
    variant: u8,
    revision: u8,
}

impl SystemControlBlock {
    pub fn new(dev: &'static SystemControlBlockDevice) -> SystemControlBlock {
        SystemControlBlock { device: dev }
    }

    pub fn cpuid(&self) -> CpuId {
        let cpuid = self.device.cpuid.get();
        CpuId {
            part_no: (cpuid >> 4) as u16 & 0xfff,
            implementer: (cpuid >> 24) as u8,
            variant: (cpuid >> 20) as u8 & 0xf,
            revision: cpuid as u8 & 0xf,
        }
    }

    pub fn set_pendsv(&self) {
        let val = self.device.icsr.get();
        self.device.icsr.set(val | (1 << 28));
    }
}
