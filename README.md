# roulios

A minimal kernel implemented in Rust and targeting ARM embedded devices (for now.)

## Build

Build is relying on Cargo, Rust nightly and an ARM GCC toolchain. To build on Ubuntu, first install an ARM GCC toolchain:

    sudo apt-get install gcc-arm-none-eabi

Then simply run:

    make

## Run on qemu

At the moment only the STM32-P103 from Olimex board is supported. If you don't have that board,
you can still run it by using a specific version of Qemu found here:

    ./configure --disable-werror --enable-debug \
        --target-list="arm-softmmu" \
        --extra-cflags=-DSTM32_UART_NO_BAUD_DELAY \
        --extra-cflags=-DSTM32_UART_ENABLE_OVERRUN \
        --disable-gtk
    make

Then:

    make run_qemu
    
A .gdbinit file is available to debug using gdb:

    make debug_qemu
    arm-none-eabi-gdb
