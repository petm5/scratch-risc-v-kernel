MEMORY {
  program (rwx) : ORIGIN = 0x80000000, LENGTH = 0x10000
  data (rw) : ORIGIN = 0x80010000, LENGTH = 0x30000
  video (w) : ORIGIN = 0x80040000, LENGTH = 0x4000
}

SECTIONS {
  .text : {
    *(.text.init)
    *(.text .text.*)
  } > program :text

  PROVIDE(_global_pointer = .);

  .rodata : {
    *(.rodata .rodata.*)
  } > data :data

  .data : {
    *(.sdata .sdata.*) *(.data .data.*)
  } > data :data

  .bss : {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } > data :bss

  . = ALIGN(16);
  PROVIDE(_init_stack_top = . + 0x1000);
}
