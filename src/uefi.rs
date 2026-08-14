// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

static IMAGE_HANDLE: AtomicPtr<ImageHandle> = AtomicPtr::new(ptr::null_mut());
static SYSTEM_TABLE: AtomicPtr<SystemTable> = AtomicPtr::new(ptr::null_mut());

#[repr(C)]
pub struct ImageHandle {}

#[repr(C)]
pub struct SystemTable {}

pub unsafe fn setup(image_handle: *mut ImageHandle, system_table: *mut SystemTable) {
    IMAGE_HANDLE.store(image_handle, Ordering::Relaxed);
    SYSTEM_TABLE.store(system_table, Ordering::Relaxed);
}
