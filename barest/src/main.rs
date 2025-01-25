#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Example: Write "Hello, World!" to VGA text buffer
    let framebuffer = 0xb8000 as *mut u8;
    let message = b"Hello, World!";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *framebuffer.offset(i as isize * 2) = byte;
            *framebuffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
