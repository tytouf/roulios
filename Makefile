BOARD=stm32
PREFIX=arm-none-eabi
TARGET=cortex-m3

OBJ_DIR=target/$(TARGET)/release

all: rlibs $(OBJ_DIR)/nk.bin

clean:
	cargo clean

$(OBJ_DIR)/nk.elf: target/$(TARGET)/release/libroulios.a
	$(PREFIX)-ld \
	--gc-sections \
	-T scripts/stm32-p103/layout.ld \
	-o $@ \
	$<
	size $@

$(OBJ_DIR)/nk.bin: $(OBJ_DIR)/nk.elf
	$(PREFIX)-objcopy \
		-O binary \
		$< \
		$@

rlibs:
	cargo build --target ./targets/$(TARGET).json --release --features=board_$(BOARD) --verbose

run_qemu:
	qemu-system-arm -M stm32-p103 -nographic -kernel target/$(TARGET)/release/nk.bin

debug_qemu:
	qemu-system-arm -s -S -M stm32-p103 -nographic -kernel target/$(TARGET)/release/nk.bin
