MEMORY {
  program (rwx) : ORIGIN = 0x80000000, LENGTH = 100000
}

SECTIONS {
  .text : {
    *(.text.init)
    *(.text .text.*)
  } > program :text

  PROVIDE(_global_pointer = .);

  .rodata : {
    *(.rodata .rodata.*)
  } > program :text

  .data : {
    *(.sdata .sdata.*) *(.data .data.*)
  } > program :data

  .bss : {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } > program :bss

  . = ALIGN(16);
  PROVIDE(_init_stack_top = . + 0x1000);
}
