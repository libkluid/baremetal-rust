static IO_PORT_A: usize = 0x40010800;
static IO_PORT_B: usize = 0x40010C00;
static IO_PORT_C: usize = 0x40011000;
static IO_PORT_D: usize = 0x40011400;
static IO_PORT_E: usize = 0x40011800;

pub unsafe fn config_led() {
    (0x40021018 as *mut u32).write_volatile(1 << 2);

    const OFFSET: usize = 0x00;
    const REG: *mut u32 = (IO_PORT_A + OFFSET) as *mut u32;

    let bits = REG.read_volatile();
    let mask = 0xFF0FFFFF;
    REG.write_volatile(bits & mask | 0x00300000);
}

pub unsafe fn led_on() {
    const OFFSET: usize = 0x10;
    const REG: *mut u32 = (IO_PORT_A + OFFSET) as *mut u32;

    REG.write_volatile(0x1 << 5);
}
