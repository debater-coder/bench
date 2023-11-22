#![no_main]
#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let stdout = system_table.stdout();
    stdout.clear().unwrap();
    stdout.write_str("Hello, World!").unwrap();

    loop {}
}