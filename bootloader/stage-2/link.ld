/**! Linker Script passed to the compiler, configured in .cargo/config.toml.  */

/* Specify executable entry point (first section) */
ENTRY(_start)

SECTIONS{
    /*  Place the .text._start section at the very start of the output binary,
        followed by all the other .text sections. */
    . = 0x0;
    .text : {
      *(.text._start)
      *(.text*)
    }

    /* Force the .rodata section after the start section. */
    .rodata : {
        *(.rodata .rodata.*)
    }

    /* Force the .eh_frame_hdr and .eh_frame section after the start section. */
    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }

    /* Remove ELF metadata from the output file. */
    /DISCARD/ : { *(.comment) }
}