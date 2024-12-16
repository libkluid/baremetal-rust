MEMORY
{
  BOOT2(rx) : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH(rx) : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  SRAM(rwx) : ORIGIN = 0x20000000, LENGTH = 256K
  SCRATCH_X(rwx) : ORIGIN = 0x20040000, LENGTH = 4K
  SCRATCH_Y(rwx) : ORIGIN = 0x40041000, LENGTH = 4K
}

ENTRY(RESET)
EXTERN(RESET_VECTOR)

SECTIONS {
  .boot2 ORIGIN(BOOT2) :
  {
    KEEP(*(.boot2))
  } > BOOT2

  .vector_table ORIGIN(FLASH) :
  {
    /* First Entry: initial stack Pointer value */
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  .text :
  {
    *(.text .text.*)
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    *(.bss .bss.*);
  } > SRAM

  .data :
  {
    *(.data .data.*);
  } > SRAM

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*)
  }
}
