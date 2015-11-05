/*
 * General Purpose Input Output
 *
 * STM32 Reference Manual, Chapter 9.
 *
 */

use volatile::RW;

#[repr(C)]
pub struct GpioDevice {
    /* 0x00 */ pub crl: RW<u32>,
    /* 0x04 */ pub crh: RW<u32>,
    /* 0x08 */ pub idr: RW<u32>,
    /* 0x0c */ pub odr: RW<u32>,
    /* 0x10 */ pub bsrr: RW<u32>,
    /* 0x14 */ pub brr: RW<u32>,
    /* 0x18 */ pub lckr: RW<u32>,
}

pub struct Gpio {
    device: &'static GpioDevice,
}

pub enum Pin {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
    P8 = 8,
    P9 = 9,
    P10 = 10,
    P11 = 11,
    P12 = 12,
    P13 = 13,
    P14 = 14,
    P15 = 15,
}

pub enum Speed {
    _10MHz = 0x1,
    _2MHz = 0x2,
    _50MHz = 0x3,
}

pub enum OutputMode {
    GeneralPushPull = 0x0,
    GeneralOpenDrain = 0x1,
    AlternatePushPull = 0x2,
    AlternateOpenDrain = 0x3,
}

pub enum InputMode {
    Analog = 0x0,
    Floating = 0x1,
    PullUpDown = 0x2,
}

impl Gpio {
    pub fn new(dev: &'static GpioDevice) -> Gpio {
        Gpio { device: dev }
    }

    pub fn set_pin_as_input(&self, pin: Pin, mode: InputMode) {
        let mut n = pin as u8;
        if n <= 7 {
            n = n * 4;
            let mut x = self.device.crl.get();
            x &= !(0xf << n); // clear
            x |= (mode as u32) << n;
            self.device.crl.set(x);
        } else {
            n = n - 8;
            n = n * 4;
            let mut x = self.device.crh.get();
            x &= !(0xf << n); // clear
            x |= (mode as u32) << n;
            self.device.crh.set(x);
        }
    }

    pub fn set_pin_as_output(&self, pin: Pin, speed: Speed, mode: OutputMode) {
        let mut n = pin as u8;
        if n <= 7 {
            n = n * 4;
            let mut x = self.device.crl.get();
            x &= !(0xf << n); // clear
            x |= (speed as u32) << (n + 2);
            x |= (mode as u32) << n;
            self.device.crl.set(x);
        } else {
            n = n - 8;
            n = n * 4;
            let mut x = self.device.crh.get();
            x &= !(0xf << n); // clear
            x |= (speed as u32) << (n + 2);
            x |= (mode as u32) << n;
            self.device.crh.set(x);
        }
    }


}
