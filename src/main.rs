// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod arch;
mod uefi;

#[unsafe(no_mangle)]
pub unsafe extern "efiapi" fn efi_main(
    image_handle: *mut uefi::ImageHandle,
    system_table: *mut uefi::SystemTable,
) -> ! {
    unsafe {
        uefi::setup(image_handle, system_table);
    }

    unsafe {
        kernel_main();
    }
}

unsafe extern "sysv64" fn kernel_main() -> ! {
    loop {}
}

#[panic_handler]
fn on_panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
