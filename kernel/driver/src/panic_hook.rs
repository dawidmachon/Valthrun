use core::panic::PanicInfo;

use winapi::km::wdm::DbgPrintEx;

use crate::kdef::{DPFLTR_LEVEL, KeBugCheck};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        DbgPrintEx(0, DPFLTR_LEVEL::ERROR as u32, "[VT] Driver paniced. Trigger BugCheck.\n\0".as_ptr());
        KeBugCheck(1);
    }
}

/// Explanation can be found here: https://github.com/Trantect/win_driver_example/issues/4
#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

/// When using the alloc crate it seems like it does some unwinding. Adding this
/// export satisfies the compiler but may introduce undefined behaviour when a
/// panic occurs.
/// 
/// Source: https://github.com/memN0ps/rootkit-rs/blob/da9a9d04b18fea58870aa1dbb71eaf977b923664/driver/src/lib.rs#L32
#[no_mangle]
extern "C" fn __CxxFrameHandler3() -> ! {
    unsafe {
        DbgPrintEx(0, DPFLTR_LEVEL::ERROR as u32, "[VT] __CxxFrameHandler3 has been called. This should no occur.\n\0".as_ptr());
        KeBugCheck(1);
    }
}

// #[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }