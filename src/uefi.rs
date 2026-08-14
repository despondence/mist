// Copyright (C) 2026 rain (despondence) <308736589+despondence@users.noreply.github.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::sync::atomic::{AtomicPtr, Ordering};
use core::{ffi, ptr};

static IMAGE_HANDLE: AtomicPtr<ImageHandle> = AtomicPtr::new(ptr::null_mut());
static SYSTEM_TABLE: AtomicPtr<SystemTable> = AtomicPtr::new(ptr::null_mut());

pub fn setup(image_handle: *mut ImageHandle, system_table: *mut SystemTable) {
    IMAGE_HANDLE.store(image_handle, Ordering::Relaxed);
    SYSTEM_TABLE.store(system_table, Ordering::Relaxed);
}

pub fn image_handle() -> *mut ImageHandle {
    IMAGE_HANDLE.load(Ordering::Relaxed)
}

pub fn system_table() -> *mut SystemTable {
    SYSTEM_TABLE.load(Ordering::Relaxed)
}

pub fn boot_services() -> *mut BootServices {
    let system_table = system_table();

    if system_table.is_null() {
        ptr::null_mut()
    } else {
        unsafe { (*system_table).boot_services }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Status {
    value: usize,
}

impl Status {
    const ERROR_MASK: usize = isize::MIN.cast_unsigned();

    pub const SUCCESS: Self = Self::new(0);
    pub const LOAD_ERROR: Self = Self::new_error(1);
    pub const INVALID_PARAMETER: Self = Self::new_error(2);
    pub const UNSUPPORTED: Self = Self::new_error(3);
    pub const BUFFER_TOO_SMALL: Self = Self::new_error(5);
    pub const NOT_FOUND: Self = Self::new_error(14);

    const fn new(value: usize) -> Self {
        Self { value }
    }

    const fn new_error(value: usize) -> Self {
        Self::new(value | Self::ERROR_MASK)
    }

    const fn is_error(self) -> bool {
        self.value & Self::ERROR_MASK == Self::ERROR_MASK
    }
}

#[repr(C)]
pub struct ImageHandle {}

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: *mut ffi::c_void,
    pub con_in: *mut ffi::c_void,
    pub console_out_handle: *mut ffi::c_void,
    pub con_out: *mut ffi::c_void,
    pub standard_error_handle: *mut ffi::c_void,
    pub std_err: *mut ffi::c_void,
    pub runtime_services: *mut ffi::c_void,
    pub boot_services: *mut BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut ffi::c_void,
}

pub const ALLOCATE_ANY_PAGES: i32 = 0;
pub const ALLOCATE_MAX_ADDRESS: i32 = 1;

pub const LOADER_DATA: u32 = 2;

pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;

#[repr(C)]
pub struct MemoryDescriptor {
    pub r#type: u32,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub const fn from_array(array: [u8; 16]) -> Self {
        let (data1, rest) = array.split_first_chunk().unwrap();
        let (data2, rest) = rest.split_first_chunk().unwrap();
        let (data3, rest) = rest.split_first_chunk().unwrap();
        let data4 = rest.as_array().unwrap();

        Self {
            data1: u32::from_le_bytes(*data1),
            data2: u16::from_le_bytes(*data2),
            data3: u16::from_le_bytes(*data3),
            data4: *data4,
        }
    }
}

#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,
    pub raise_tpl: *const ffi::c_void,
    pub restore_tpl: *const ffi::c_void,
    pub allocate_pages: unsafe extern "efiapi" fn(
        alloc_type: i32,
        mem_type: u32,
        pages: usize,
        memory: *mut u64,
    ) -> Status,
    pub free_pages: unsafe extern "efiapi" fn(memory: u64, pages: usize) -> Status,
    pub get_memory_map: unsafe extern "efiapi" fn(
        memory_map_size: *mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> Status,
    pub allocate_pool: unsafe extern "efiapi" fn(
        pool_type: u32,
        size: usize,
        buffer: *mut *mut ffi::c_void,
    ) -> Status,
    pub free_pool: unsafe extern "efiapi" fn(buffer: *mut ffi::c_void) -> Status,
    pub create_event: *const ffi::c_void,
    pub set_timer: *const ffi::c_void,
    pub wait_for_event: *const ffi::c_void,
    pub signal_event: *const ffi::c_void,
    pub close_event: *const ffi::c_void,
    pub check_event: *const ffi::c_void,
    pub install_protocol_interface: *const ffi::c_void,
    pub uninstall_protocol_interface: *const ffi::c_void,
    pub handle_protocol: *const ffi::c_void,
    pub reserved: *const ffi::c_void,
    pub register_protocol_notify: *const ffi::c_void,
    pub locate_handle: *const ffi::c_void,
    pub locate_device_path: *const ffi::c_void,
    pub install_configuration_table_access: *const ffi::c_void,
    pub image_load: *const ffi::c_void,
    pub image_start: *const ffi::c_void,
    pub exit: *const ffi::c_void,
    pub image_unload: *const ffi::c_void,
    pub exit_boot_services:
        unsafe extern "efiapi" fn(image_handle: *mut ImageHandle, map_key: usize) -> Status,
    pub get_next_monotonic_count: *const ffi::c_void,
    pub stall: *const ffi::c_void,
    pub set_watchdog_timer: *const ffi::c_void,
    pub connect_controller: *const ffi::c_void,
    pub disconnect_controller: *const ffi::c_void,
    pub open_protocol: *const ffi::c_void,
    pub close_protocol: *const ffi::c_void,
    pub open_protocol_information: *const ffi::c_void,
    pub protocols_per_handle: *const ffi::c_void,
    pub locate_handle_buffer: *const ffi::c_void,
    pub locate_protocol: unsafe extern "efiapi" fn(
        protocol: *mut Guid,
        registration: *mut ffi::c_void,
        interface: *mut *mut ffi::c_void,
    ) -> Status,
}
