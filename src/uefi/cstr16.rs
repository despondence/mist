// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::{iter, ptr};

pub const NUL: u16 = 0;

#[repr(transparent)]
pub struct CStr16 {
    value: [u16],
}

impl CStr16 {
    pub const unsafe fn from_codes_with_nul_unchecked(codes: &[u16]) -> &CStr16 {
        unsafe { &*(ptr::from_ref(codes) as *const CStr16) }
    }

    pub const fn as_ptr(&self) -> *const u16 {
        self.value.as_ptr()
    }
}

pub fn with_cstr16<E>(
    input: &str,
    mut with_cstr16: impl FnMut(&CStr16) -> Result<(), E>,
) -> Result<(), E> {
    let mut codes = [0; 256];
    let mut iter = input.encode_utf16();

    loop {
        let count = iter
            .by_ref()
            .take(codes.len() - 1)
            .chain(iter::once(NUL))
            .zip(&mut codes)
            .map(|(source, destination)| *destination = source)
            .count();

        let cstr16 = unsafe { CStr16::from_codes_with_nul_unchecked(&codes[..count]) };

        if let Err(error) = (with_cstr16)(cstr16) {
            return Err(error);
        }

        if count != codes.len() {
            return Ok(());
        }
    }
}
