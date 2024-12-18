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
    let sdata = &_sdata as *const u32 as usize;
    let edata = &_edata as *const u32 as usize;

    let lma_src = &_sidata as *const u32;
    let mut vma_est = sdata as *mut u32;

    while (vma_est as usize) < edata {
        let word = lma_src.read();
        vma_est.write(word);
        vma_est = vma_est.offset(1);
    }

    let sbss = &_sbss as *const u32 as usize;
    let ebss = &_ebss as *const u32 as usize;

    let mut b = sbss as *mut u32;

    while (b as usize) < ebss {
        b.write(0);
        b = b.offset(1);
    }
}
