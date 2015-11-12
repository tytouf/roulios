/*
 * Reset & Clock Control (RCC)
 *
 * STM32 Reference Manual, Chapter 7.
 *
 */

use volatile::RW;

#[repr(C)]
pub struct RccDevice {
    /* 0x00 */ pub cr: RW<u32>,
    /* 0x04 */ pub cfgr: RW<u32>,
    /* 0x08 */ pub cir: RW<u32>,
    /* 0x0C */ pub apb2rstr: RW<u32>,
    /* 0x10 */ pub apb1rstr: RW<u32>,
    /* 0x14 */ pub ahbenr: RW<u32>,
    /* 0x18 */ pub apb2enr: RW<u32>,
    /* 0x1C */ pub apb1enr: RW<u32>,
    /* 0x20 */ pub bdcr: RW<u32>,
    /* 0x24 */ pub csr: RW<u32>,
}

pub struct Rcc {
    device: &'static RccDevice,
}

pub enum ControlRegister {
    HSION = 1 << 0,
    HSIRDY = 1 << 1,
}

pub enum Apb1Peripheral {
    USART2 = 1 << 17,
}

pub enum Apb2Peripheral {
    AFIO = 1 << 0,
    IOPA = 1 << 2,
}

impl Rcc {
    pub fn new(dev: &'static RccDevice) -> Rcc {
        Rcc { device: dev }
    }

    pub fn reset(&self) {
        self.device.cr.set(0x00000083);
        self.device.cfgr.set(0x00000000);
        self.device.cir.set(0x009f0000);
    }

    pub fn enable_hsi_clock(&self) {
        let x = self.device.cr.get();
        self.device.cr.set(x | ControlRegister::HSION as u32);
        // wait until HSI is stable
        while (self.device.cr.get() & ControlRegister::HSIRDY as u32) == 0 { }
    }

    pub fn enable_apb2_peripheral(&self, p: Apb2Peripheral) {
        let x = self.device.apb2enr.get();
        self.device.apb2enr.set(x | p as u32);
    }

    pub fn enable_apb1_peripheral(&self, p: Apb1Peripheral) {
        let x = self.device.apb1enr.get();
        self.device.apb1enr.set(x | p as u32);
    }
}
