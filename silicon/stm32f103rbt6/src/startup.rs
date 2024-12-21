pub unsafe fn init_clock() {
    crate::rcc::clocks::default();
}

extern "C" {
    // load address(LMA) of .data section
    static _sidata: u32;

    // start address of .data section
    static _sdata: u32;
    // end address of .data section
    static _edata: u32;

    // start address of .bss section
    static _sbss: u32;
    // end address of .bss section
    static _ebss: u32;
}

pub unsafe fn device_init() {
    let sidata = &_sidata as *const u32;
    let sdata = &_sdata as *const u32 as usize;
    let edata = &_edata as *const u32 as usize;
    let count = (edata - sdata) / core::mem::size_of::<u32>();
    core::ptr::copy_nonoverlapping::<u32>(sidata, sdata as *mut u32, count);

    let sbss = &_sbss as *const u32 as usize;
    let ebss = &_ebss as *const u32 as usize;
    let count = (ebss - sbss) / core::mem::size_of::<u32>();
    core::ptr::write_bytes(sbss as *mut u32, 0, count);
}
