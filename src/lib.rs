#![feature(lang_items)]
#![feature(no_std)]
#![feature(asm)]
#![feature(core_str_ext)]
#![no_std]

extern crate volatile;
//extern crate alloc;
//extern crate collections;

pub mod cpu;
pub mod boards;
pub mod kernel;

use cpu::cortex_m3::systick::{SystickDevice, Systick, ClockSource};
use cpu::cortex_m3::scb::{SystemControlBlockDevice, SystemControlBlock};
use boards::stm32::usart::{UsartDevice, Usart};
use boards::stm32::rcc::{RccDevice, Rcc, Apb1Peripheral, Apb2Peripheral};
use boards::stm32::gpio::{GpioDevice, Gpio, Pin, InputMode, OutputMode, Speed};
use kernel::serial::Serial;

extern {
    static __RCC__: RccDevice;
    static __USART2__: UsartDevice;
    static __GPIOA__: GpioDevice;
    static __SYSTICK__: SystickDevice;
    static __SYSCTRLBLK__: SystemControlBlockDevice;
    static __MEMPOOL__: [u8; 1024];
}

pub fn usart2() -> &'static UsartDevice { &__USART2__ }
pub fn rcc() -> &'static RccDevice { &__RCC__ }
pub fn gpioa() -> &'static GpioDevice { &__GPIOA__ }
pub fn systick() -> &'static SystickDevice { &__SYSTICK__ }
pub fn scb() -> &'static SystemControlBlockDevice { &__SYSCTRLBLK__ }

fn puts(cons: &Serial, s: &'static str) {
    for c in s.bytes() {
        cons.send_byte(c);
    }
}

#[no_mangle]
pub extern fn tick() {
    let uart = Usart::new(usart2());
    puts(&uart, "tick!\n");
}
 
pub fn start() -> ! {
    let ser = Usart::new(usart2());
    let rcc = Rcc::new(rcc());
    let gpioa = Gpio::new(gpioa());
    let systick = Systick::new(systick());
    let scb = SystemControlBlock::new(scb());

    rcc.enable_apb2_peripheral(Apb2Peripheral::AFIO);
    rcc.enable_apb2_peripheral(Apb2Peripheral::IOPA);
    rcc.enable_apb1_peripheral(Apb1Peripheral::USART2);

    gpioa.set_pin_as_input(Pin::P3, InputMode::Floating);
    gpioa.set_pin_as_output(Pin::P2, Speed::_50MHz, OutputMode::AlternatePushPull);

    ser.init();
    puts(&ser, "hello\n");

    systick.set_reload(7200000);
    systick.clear_value();
    systick.enable(true, ClockSource::Core);

    cpu::cortex_m3::enable_interrupts();

    loop { }
}
