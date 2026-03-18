OUTPUT_ARCH({{arch}})
ENTRY(_start)
BASE_ADDRESS = {{base_address}};

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    .text : ALIGN(4096){
        stext = .;
        __executable_start = .;
        *(.text.entry)
        . = ALIGN(4K);
        strampoline = . ;
        *(.text.trampoline)
        . = ALIGN(4K);
        etrampoline = . ;
        *(.text.startup)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
         __etext = .;
    }

    .rodata : ALIGN(4096){
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    . = ALIGN(4K);
    _percpu_start = .;
    _percpu_end = _percpu_start + SIZEOF(.percpu);
    .percpu 0x0 : AT(_percpu_start) {
        _percpu_load_start = .;
        *(.percpu .percpu.*)
        _percpu_load_end = .;
        . = _percpu_load_start + ALIGN(64) * {{max_cpu_num}};
    }
    . = _percpu_end;

    .data : ALIGN(4096){
        sdata = .;
        sinit = .;
        *(.init_array .init_array.*)
        einit = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4K);
        edata = .;
    }

    .bss :  ALIGN(4096){
        *(.bss.stack)
        . = ALIGN(4K);
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(4K);
        ebss = .;
    }

    .gcc_except_table : ALIGN(4K){
        kernel_gcc_except_table = .;
        *(.gcc_except_table .gcc_except_table.*)
        kernel_gcc_except_table_end = .;
    }

    .eh_frame_hdr : ALIGN(4K){
        kernel_eh_frame_hdr = .;
        __GNU_EH_FRAME_HDR = .;
        *(.eh_frame_hdr .eh_frame_hdr.*)
        . = ALIGN(4K);
        kernel_eh_frame_hdr_end = .;
    }
    .eh_frame : ALIGN(4K){
        kernel_eh_frame = .;
        KEEP(*(.eh_frame .eh_frame.*))
        kernel_eh_frame_end = .;
    }
    . = ALIGN(4K);
    sheap = .;
    PROVIDE(end = .);
}