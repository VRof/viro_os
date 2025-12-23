#![no_std] //standart library won't work because there is no OS ;)
#![no_main] //no main func, we will use _start

#[unsafe(no_mangle)] //save _start name after compilation instead of giving it some random name like 12fsdf43
pub extern "C" fn _start() -> ! {
    loop {}
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
