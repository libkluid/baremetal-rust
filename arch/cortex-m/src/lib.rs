#![no_std]

mod macros;

unsafe extern "Rust" {
    unsafe fn main() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn RESET() -> ! {
    main();
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = RESET;
