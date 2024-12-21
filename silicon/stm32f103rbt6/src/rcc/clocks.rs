const RCC_BASE: usize = 0x4002_1000;

// SYSCLK <= 72MHz
// PCLK1 <= 36MHz
// PCLK2 <= 72MHz
// HCLK <= 72MHz
// ADCPRE <= 14MHz
pub unsafe fn default() {
    const RCC_CR: *mut u32 = (RCC_BASE + 0x00) as *mut u32;
    const RCC_CFGR: *mut u32 = (RCC_BASE + 0x04) as *mut u32;
    const RCC_CIR: *mut u32 = (RCC_BASE + 0x08) as *mut u32;

    // HSI ON(8MHz)
    RCC_CR.write_volatile(0x1 << 0);

    // Reset SW, SWS, HPRE, PRE1, PRE2, ADCPRE and MCO bits
    // SYSCLK = HSI(8MHz)
    // HCLK = SYSCLK / 1
    // PCLK2 = HCLK / 1
    // PCLK1 = HCLK / 1
    // ADCPRE = PCLK2 / 2
    // MCO = No clock
    let bits = RCC_CFGR.read_volatile();
    let mask = 0xF0FF0000; // MCO,
    RCC_CFGR.write_volatile(bits & mask);

    // Reset HSEON, CSSON and PLLON bits
    let bits = RCC_CR.read_volatile();
    let mask = 0xFEF6FFFF;
    RCC_CR.write_volatile(bits & mask);

    // Reset HSEBYP bit
    let bits = RCC_CR.read_volatile();
    let mask = 0xFFFBFFFF;
    RCC_CR.write_volatile(bits & mask);

    // Reset PLLSRC, PLLXTPRE, PLLMUL and USBPRE/OTGFSPRE bits
    let bits = RCC_CFGR.read_volatile();
    let mask = 0xFF80FFFF;
    RCC_CFGR.write_volatile(bits & mask);

    // Disable all interrupts and clear pending bits
    RCC_CIR.write_volatile(0x009F0000);
}
