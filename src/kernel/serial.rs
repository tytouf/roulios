
pub enum BaudRate {
    _9600 = 9600,
    _19200 = 19200,
    _38400 = 38400,
    _115200 = 115200,
}

pub trait Serial {
    fn read_byte(&self) -> Option<u8>;
    fn send_byte(&self, b: u8) -> Option<u8>;
    fn set_baudrate(&self, rate: BaudRate) -> Option<u32>; // use Result?
}
