ENTRY(reset_handler)

INCLUDE scripts/stm32-p103/registers.ld

MEMORY
{
  ROM(RX)   : ORIGIN = 0x08000000, LENGTH = 128K
  RAM(WAIL) : ORIGIN = 0x20000000, LENGTH = 8K
}

__data_load = LOADADDR(.data);
__STACK_TOP__ = ORIGIN(RAM) + LENGTH(RAM);

REGION_ALIAS("vectors", ROM);

SECTIONS
{
  .vector : ALIGN(4)
  {
      FILL(0xff)
      KEEP(*(.exception_vector))
      KEEP(*(.interrupt_vector))
  } > vectors

  .text : ALIGN(4)
  {
    __text_start = .;
    FILL(0xff)
    *(.text*)
    *(.rodata*)
    __text_end = .;
  } > ROM

  .data : ALIGN(4)
  {
    __data_start = .;
    *(SORT_BY_ALIGNMENT(.data*))
    . = ALIGN(4);
    __data_end = .;
  } > RAM AT>ROM =0xff

  .bss :
  {
    __bss_start = .;
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
    __bss_end = .;
    . = ALIGN(8);
    __MEMPOOL__ = .;
  } > RAM

  /DISCARD/ :
  {
    *(.rel.*)
    *(.ARM.exidx*)
    *(.ARM.extab*)
  }

}
