const RCC_BASE: usize = 0x4002_1000;
const BIT_MASK: u32 = 0x3E7EC83F;

const ACC_APB1RSTR: usize = 0x10;

#[inline]
pub unsafe fn reset() {
    const REG: *mut u32 = (RCC_BASE + ACC_APB1RSTR) as *mut u32;

    let bits = REG.read_volatile();
    REG.write_volatile(bits | BIT_MASK);
}

#[inline]
pub unsafe fn set() {
    const REG: *mut u32 = (RCC_BASE + ACC_APB1RSTR) as *mut u32;

    let bits = REG.read_volatile();
    REG.write_volatile(bits & BIT_MASK.reverse_bits());
}
