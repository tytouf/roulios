
use volatile::RW;
use kernel::serial::{Serial, BaudRate};

#[repr(C)]
pub struct UsartDevice {
    /* 0x00 */ pub sr: RW<u32>,
    /* 0x04 */ pub dr: RW<u32>,
    /* 0x08 */ pub brr: RW<u32>,
    /* 0x0C */ pub cr1: RW<u32>,
    /* 0x10 */ pub cr2: RW<u32>,
    /* 0x14 */ pub cr3: RW<u32>,
    /* 0x18 */ gtpr: RW<u32>,
}

pub struct Usart {
    device: &'static UsartDevice,
}

impl Usart {
    pub fn new(dev: &'static UsartDevice) -> Usart {
        Usart { device: dev }
    }

    pub fn init(&self) {
        self.device.cr1.set(0x0000000C);
        self.device.cr1.set(0x0000200C);
    }

    fn can_tx(&self) -> bool {
        let val = self.device.sr.get();
        (val & 0x80) != 0
    }
}

impl Serial for Usart {
    fn send_byte(&self, b: u8) -> Option<u8> {
        if self.can_tx() {
            self.device.dr.set(b as u32);
            Some(b)
        } else {
            None
        }
    }

    fn read_byte(&self) -> Option<u8> {
        None
    }

    fn set_baudrate(&self, rate: BaudRate) -> Option<u32> {
        Some(rate as u32)
    }
}
