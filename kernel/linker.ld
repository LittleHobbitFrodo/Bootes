/* 
 * linker.ld - Defines the memory layout for our simple kernel image.
 */

ENTRY(_start);    /* Tell the linker where execution begins (our assembly stub entry). */

SECTIONS {
  /* We define the start address of the entire kernel in physical/virtual memory. */
  . = 0x10000; /* Start loading our code at this fixed virtual address */
  
  /* The .text section holds the executable instructions (the code). */
  .text : {
    *(.text)
  }

  /* The .data section holds initialized global variables. */
  .data : {
    *(.data)
  }

  /* BSS is for uninitialized data, which must be allocated but not written to. */
  .bss : {
    *(.bss)
  }
}
