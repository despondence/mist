// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::arch;

use crate::arch::x86_64::page::PhysAddr;

pub mod page;

pub struct Thread;

pub struct Ring3;

pub struct Cr0;
pub struct Cr1;
pub struct Cr3;
pub struct Cr4;

pub trait ReadReg {
    type Value;

    unsafe fn read_reg() -> Self::Value;
}

pub trait WriteReg {
    type Value;

    unsafe fn write_reg(value: Self::Value);
}

impl WriteReg for Cr3 {
    type Value = PhysAddr;

    unsafe fn write_reg(pml4_addr: Self::Value) {
        unsafe {
            arch::asm!(
                "mov cr3, {}",
                in(reg) pml4_addr.to_u64(),
                options(nostack, preserves_flags),
            );
        }
    }
}

impl Thread {
    pub unsafe fn read_reg<R: ReadReg>(&mut self) -> R::Value {
        unsafe { R::read_reg() }
    }

    pub unsafe fn write_reg<R: WriteReg>(&mut self, value: R::Value) {
        unsafe { R::write_reg(value) }
    }
}
