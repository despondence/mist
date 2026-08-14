// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableFlags {
    value: u64,
}

impl PageTableFlags {
    pub const PRESENT: Self = Self::from_bits_retain(1 << 0);
    pub const WRITABLE: Self = Self::from_bits_retain(1 << 1);
    pub const USER_ACCESSIBLE: Self = Self::from_bits_retain(1 << 2);
    pub const WRITE_THROUGH: Self = Self::from_bits_retain(1 << 3);
    pub const NO_CACHE: Self = Self::from_bits_retain(1 << 4);
    pub const ACCESSED: Self = Self::from_bits_retain(1 << 5);
    pub const DIRTY: Self = Self::from_bits_retain(1 << 6);
    pub const HUGE_PAGE: Self = Self::from_bits_retain(1 << 7);
    pub const GLOBAL: Self = Self::from_bits_retain(1 << 8);
    pub const NO_EXECUTE: Self = Self::from_bits_retain(1 << 63);

    pub const fn from_bits_retain(bits: u64) -> Self {
        Self { value: bits }
    }

    pub const fn bits(self) -> u64 {
        self.value
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & other.bits())
    }

    pub const fn union(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() | other.bits())
    }

    pub const fn contains(self, other: Self) -> bool {
        self.intersection(other).bits() == other.bits()
    }
}
