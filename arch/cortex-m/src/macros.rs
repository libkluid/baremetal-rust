#[macro_export]
macro_rules! main {
    ($path: path) => {
        #[export_name = "main"]
        pub unsafe extern "C" fn __cortex_m_main() -> ! {
            let entry: unsafe fn() -> ! = $path;
            entry()
        }
    };
}
