#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use test_kernel_higher_half::{exit_qemu, QemuExitCode, BOOTLOADER_CONFIG};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    exit_qemu(QemuExitCode::Success);
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
}
