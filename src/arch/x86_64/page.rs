// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysAddr {
    value: u64,
}

impl PhysAddr {
    pub const fn new(addr: u64) -> Self {
        Self { value: addr }
    }

    pub const fn to_u64(self) -> u64 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PageTableEntry {
    value: u64,
}

impl PageTableEntry {
    const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;
    const FLAGS_MASK: u64 = !Self::ADDR_MASK;

    pub const UNUSED: Self = Self { value: 0 };

    pub const fn set_unused(&mut self) {
        *self = Self::UNUSED;
    }

    pub const fn flags(self) -> PageTableFlags {
        PageTableFlags::from_bits_retain(self.value & Self::FLAGS_MASK)
    }

    pub const fn phys_addr(self) -> PhysAddr {
        PhysAddr::new(self.value & Self::ADDR_MASK)
    }

    pub const fn set(&mut self, phys_addr: PhysAddr, flags: PageTableFlags) {
        let phys_addr = phys_addr.to_u64() & Self::ADDR_MASK;
        let flags = flags.bits() & Self::FLAGS_MASK;

        self.value = phys_addr | flags;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(align(4096), C)]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const EMPTY: Self = Self {
        entries: [PageTableEntry::UNUSED; _],
    };

    pub const fn entries(&self) -> &[PageTableEntry; 512] {
        &self.entries
    }

    pub const fn entries_mut(&mut self) -> &mut [PageTableEntry; 512] {
        &mut self.entries
    }
}
