// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::arch;

use crate::arch::x86_64::Thread;

mod sealed {
    pub trait Sealed {
        unsafe fn read(addr: u16) -> Self;
        unsafe fn write(addr: u16, value: Self);
    }
}

pub unsafe trait PortSafe: sealed::Sealed {}

macro_rules! port_safe {
    ($($ty:ty => $reg:tt,)*) => {$(
        impl sealed::Sealed for $ty {
            #[inline]
            unsafe fn read(addr: u16) -> Self {
                let value;

                unsafe {
                    arch::asm!(
                        concat!("in ", $reg, ", dx"),
                        in("dx") addr,
                        out($reg) value,
                        options(nomem, nostack, preserves_flags),
                    );
                }

                value
            }

            #[inline]
            unsafe fn write(addr: u16, value: Self) {
                unsafe {
                    arch::asm!(
                        concat!("out dx, ", $reg),
                        in("dx") addr,
                        in($reg) value,
                        options(nomem, nostack, preserves_flags),
                    );
                }
            }
        }

        unsafe impl PortSafe for $ty {}
    )*};
}

port_safe! {
    u8 => "al",
    u16 => "ax",
    u32 => "eax",
}

impl Thread {
    pub fn pmio_read<T: PortSafe>(&self, addr: u16) -> T {
        unsafe { T::read(addr) }
    }

    pub fn pmio_write<T: PortSafe>(&self, addr: u16, value: T) {
        unsafe { T::write(addr, value) }
    }
}
