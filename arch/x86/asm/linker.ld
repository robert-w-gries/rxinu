/* The bootloader will look at this image and start execution at the symbol
   designated as the entry point. */
ENTRY(startup)

/* Tell where the various sections of the object files will be put in the final
   kernel image. */
SECTIONS
{
	/* Begin putting sections at 1 MiB, a conventional place for kernels to be
	   loaded at by the bootloader. */
	. = 1M;

	/* First put the multiboot header, as it is required to be put very early
	   early in the image or the bootloader won't recognize the file format.
	   Next we'll put the .text section. */

	/* Read-only data. */
	.rodata BLOCK(4K) :
	{
		KEEP(*(.multiboot_header))
		*(.rodata .rodata.*)
                . = ALIGN(4K);
	}

	.text BLOCK(4K) :
	{
		*(.text .text.*)
                . = ALIGN(4K);
	}

	/* Read-write data (initialized) */
	.data BLOCK(4K) :
	{
		*(.data .data.*)
                . = ALIGN(4K);
	}

	/* Read-write data (uninitialized) and stack */
	.bss BLOCK(4K) :
	{
		*(.bss .bss.*)
                . = ALIGN(4K);
	}

	.got BLOCK(4K) :
	{
		*(.got)
                . = ALIGN(4K);
	}

	.got.plt BLOCK(4K) :
	{
		*(.got.plt)
                . = ALIGN(4K);
	}

	.data.rel.ro BLOCK(4K) :
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
                . = ALIGN(4K);
	}

	/* The compiler may produce other sections, by default it will put them in
	   a segment with the same name. Simply add stuff here as needed. */
}
