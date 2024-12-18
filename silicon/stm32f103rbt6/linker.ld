MEMORY
{
  FLASH(rx) : ORIGIN = 0x08000000, LENGTH = 128K
  SRAM(rwx) : ORIGIN = 0x20000000, LENGTH = 20K
}

ENTRY(RESET)
EXTERN(RESET_VECTOR)

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First Entry: initial stack Pointer value */
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  .text ALIGN(4) :
  {
    *(.text)
    *(.text*)

    KEEP(*(.init))
    KEEP(*(.fini))

    . = ALIGN(4);
    _etext = .; 
  } > FLASH

  .rodata ALIGN(4) :
  {
    *(.rodata)
    *(.rodata*)

    . = ALIGN(4);
    _erodata = .;
  } > FLASH

  _sidata = LOADADDR(.data);
  .data ALIGN(4) :
  {
    _sdata = .;
    *(.data)
    *(.data*)

    . = ALIGN(4);
    _edata = .;
  } > SRAM AT > FLASH

  .bss ALIGN(4) :
  {
    _sbss = .;
    *(.bss)
    *(.bss*)

    . = ALIGN(4);
    _ebss = .;
  } > SRAM
}
