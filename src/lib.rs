// This create is its own allocator
#![feature(allocator)]
#![allocator]
#![feature(collections)]
#![feature(no_std)]
#![feature(alloc)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(ptr_as_ref)]
#![feature(fnbox)]
#![no_std]

extern crate volatile;
extern crate alloc;
extern crate collections;

#[macro_use]
pub mod cpu;
pub mod boards;
pub mod kernel;
pub mod rt;

use alloc::boxed::Box;
use cpu::cortex_m3::systick::{SystickDevice, Systick, ClockSource};
use cpu::cortex_m3::scb::{SystemControlBlockDevice, SystemControlBlock};
use boards::stm32::usart::{UsartDevice, Usart};
use boards::stm32::rcc::{RccDevice, Rcc, Apb1Peripheral, Apb2Peripheral};
use boards::stm32::gpio::{GpioDevice, Gpio, Pin, InputMode, OutputMode, Speed};
use kernel::serial::Serial;
use collections::Vec;

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
pub fn tick() {
    let uart = Usart::new(usart2());
    puts(&uart, "tick!\n");
}

/*
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() {
    loop { }
}
*/

#[allow(dead_code)]
struct TestAlloc {
    abc: u32,
    def: u8,
}

fn test_alloc() -> Vec<u32> {
    let mut counter: Box<u32> = Box::new(0);
    let ta: Box<TestAlloc> = Box::new(TestAlloc { abc: 1, def: 1 });

    while *counter < 1000 {
        *counter += 1;
    }

    let mut ret: Vec<u32> = Vec::new();
    ret.push(1);
    ret.push(2);
    ret.push(3);
    ret
}

/*
fn wait_a_bit() {
    let mut counter = 0;
    while counter < 0x100000 {
        counter += 1;
        cpu::cortex_m3::nop();
    }
}
*/

fn task_1() {
    let uart = Usart::new(usart2());
    loop {
        puts(&uart, "task 1!\n");
        svc!(0);
    }
}

fn task_2() {
    let uart = Usart::new(usart2());
    loop {
        puts(&uart, "task 2!\n");
        svc!(0);
    }
}

fn task_3() {
    let uart = Usart::new(usart2());
    loop {
        puts(&uart, "task 3 part 1!\n");
        svc!(0);
        puts(&uart, "task 3 part 2!\n");
        svc!(0);
        puts(&uart, "task 3 part 3!\n");
        svc!(0);
    }
}

fn task_4() {
    let uart = Usart::new(usart2());
    loop {
        puts(&uart, "task 4!\n");
        svc!(0);
    }
}

pub fn start() -> ! {
    kernel::alloc::init_allocator(&__MEMPOOL__ as *const u8 as usize,
                                  core::mem::size_of_val(&__MEMPOOL__));
    let x = test_alloc();
    let a = Box::new(32u32);

    let ks = kernel::init();

    let ser = Usart::new(usart2());
    let rcc = Rcc::new(rcc());
    let gpioa = Gpio::new(gpioa());
    let systick = Systick::new(systick());
    let scb = SystemControlBlock::new(scb());

    rcc.reset();
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


    ks.tasks.spawn_task(task_1);
    ks.tasks.spawn_task(task_2);
    ks.tasks.spawn_task(task_3);
    ks.tasks.spawn_task(move || {
        let mut counter = 0;
        loop {
            puts(&ser, "moved 4\n");
            svc!(1);
            counter += 1;
            if counter == 30 {
                loop {}
            }
        }
    });

    cpu::cortex_m3::enable_interrupts();

    cpu::cortex_m3::set_psp(ks.tasks.get_current_task().stack_ptr as u32);

    /* set PendSv to schedule idle task
     */
    scb.set_pendsv();
    loop { }
}

