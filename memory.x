/* Entry Point */
ENTRY(Reset_Handler)

/* Specify the memory areas */
MEMORY
{
RAM (xrw)      : ORIGIN = 0x20000000, LENGTH = 128K
/* CCMRAM (xrw)      : ORIGIN = 0x10000000, LENGTH = 64K */
FLASH (rx)      : ORIGIN = 0x8000000, LENGTH = 1024K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);    /* end of RAM */

/* Define output sections */
SECTIONS
{
  /* The startup code goes first into FLASH */
  .vector_table :
  {
    . = ALIGN(4);
    KEEP(*(.stacktop)) /* Startup code */
    . = ALIGN(4);
    KEEP(*(.vector_table)) /* Startup code */
    . = ALIGN(4);
  } >FLASH

  /* The program code and other data goes into FLASH */
  .text :
  {
    . = ALIGN(4);
    *(.text*)          /* .text* sections (code) */
    . = ALIGN(4);
    _etext = .;        /* define a global symbols at end of code */
  } >FLASH

  /* Constant data goes into FLASH */
  .rodata :
  {
    . = ALIGN(4);
    *(.rodata*)        /* .rodata* sections (constants, strings, etc.) */
    . = ALIGN(4);
  } >FLASH


  /* used by the startup to initialize data */
  _sidata = LOADADDR(.data);

  /* Initialized data sections goes into RAM, load LMA copy after code */
  .data : 
  {
    . = ALIGN(4);
    _sdata = .;        /* create a global symbol at data start */
    *(.data*)          /* .data* sections */

    . = ALIGN(4);
    _edata = .;        /* define a global symbol at data end */
  } >RAM AT> FLASH
  
  /* Uninitialized data section */
  . = ALIGN(4);
  .bss :
  {
    /* This is used by the startup in order to initialize the .bss secion */
    _sbss = .;         /* define a global symbol at bss start */
    __bss_start__ = _sbss;
    *(.bss*)
    *(COMMON)

    . = ALIGN(4);
    _ebss = .;         /* define a global symbol at bss end */
    __bss_end__ = _ebss;
  } >RAM
}


