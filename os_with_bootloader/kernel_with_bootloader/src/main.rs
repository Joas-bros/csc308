#![no_std]
#![no_main]

mod writers; // This will include the writers folder as a module

use writers::writer::FrameBufferWriter; // Access FrameBufferWriter from the writers module
use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
use core::fmt::Write; // Required for `writeln!`

/// Use the `entry_point` macro to register the entry point function.
/// Optionally, pass a custom configuration.
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

/// Entry point of the kernel.
fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // Get framebuffer information and mutable buffer
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    // Initialize the framebuffer writer
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    // Test: Write a simple message
    writeln!(frame_buffer_writer, "Hello, world!").unwrap();

    // Test: Write a long line to check row wrapping
    writeln!(
        frame_buffer_writer,
        "This is a very long line that should wrap to the next line automatically."
    )
    .unwrap();

    // Test: Write multiple lines to trigger screen scrolling
    for i in 0..50 {
        writeln!(frame_buffer_writer, "This is line number {}", i).unwrap();
    }

    // Test: Write a single character after scrolling
    frame_buffer_writer.write_char('A');

    // Enter an infinite loop to halt the CPU
    loop {
        hlt(); // Stop x86_64 from being unnecessarily busy while looping
    }
}

/// Panic handler to handle kernel panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt(); // Halt the CPU to prevent unnecessary activity
    }
}
