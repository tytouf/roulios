/*
 * Systick
 *
 * Cortex-m3 Reference Manual, Chapter 8.2.2 - Systick registers.
 *
 */

use volatile::RW;

#[repr(C)]
pub struct SystickDevice {
    /* 0x00 */ pub ctrl: RW<u32>,
    /* 0x04 */ pub reload: RW<u32>,
    /* 0x08 */ pub value: RW<u32>,
    /* 0x0c */ pub calib: RW<u32>,
}

pub struct Systick {
    device: &'static SystickDevice,
}

pub enum ClockSource {
    Core = 0x4,
    External = 0x0,
}

impl Systick {
    pub fn new(dev: &'static SystickDevice) -> Systick {
        Systick { device: dev }
    }

    pub fn enable(&self, tick_int: bool, clk_source: ClockSource) {
        self.device.ctrl.set(1u32
                             | clk_source as u32
                             | if tick_int { 0x2 } else { 0 });
    }

    pub fn disable(&self) {
        self.device.ctrl.set(0);
    }

    pub fn set_reload(&self, val: u32) {
        if val < 1 || val > 0x00ffffff {
            panic!();
        }
        self.device.reload.set(val);
    }

    pub fn clear_value(&self) {
        self.device.value.set(0);
    }

    pub fn read_value(&self) {
        self.device.value.get()
    }
}
