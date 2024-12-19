static RCC_BASE: usize = 0x4002_1000;

unsafe fn reset_apb1() {
    const OFFSET: usize = 0x10;
    const REG: *mut u32 = (RCC_BASE + OFFSET) as *mut u32;

    let bits = REG.read_volatile();
    let reset = 0xCAFEC9FF;
    REG.write_volatile(bits & reset);
    REG.write_volatile(reset.reverse_bits());
}

unsafe fn reset_apb2() {
    const OFFSET: usize = 0x0C;
    const REG: *mut u32 = (RCC_BASE + OFFSET) as *mut u32;

    let bits = REG.read_volatile();
    let reset = 0x0038FFFD;
    REG.write_volatile(bits & reset);
    REG.write_volatile(reset.reverse_bits());
}

unsafe fn reset_ahb() {
    const OFFSET: usize = 0x14;
    const REG: *mut u32 = (RCC_BASE + OFFSET) as *mut u32;

    let bits = REG.read_volatile();
    let reset = 0x00000557;
    REG.write_volatile(bits & reset);
    REG.write_volatile(reset.reverse_bits());
}

pub unsafe fn reset_bus() {
    reset_apb1();
    reset_apb2();
    reset_ahb();
}
