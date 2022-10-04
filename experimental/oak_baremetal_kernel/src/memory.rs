//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use core::result::Result;
use linked_list_allocator::LockedHeap;
use log::info;
use x86_64::structures::paging::{frame::PhysFrameRangeInclusive, Size2MiB};

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(test)]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initializes the global allocator from the largest contiguous slice of available memory.
///
/// Pointers to addresses in the memory area (or references to data contained within the slice) must
/// be considered invalid after calling this function, as the allocator may overwrite the data at
/// any point.
///
/// We ensure that data up address specified by `.stack_start` in the linker script is untouched,
/// even if it falls within the largest slice of memory.
///
/// Case in point, boot metadata is often stored somewhere in the memory area, so calling this takes
/// ownership of the `boot:BootInfo`, as the data provided to us by the bootloader will get
/// clobbered after initializing the heap.
pub fn init_allocator(range: PhysFrameRangeInclusive<Size2MiB>) -> Result<(), &'static str> {
    let ram_min = rust_hypervisor_firmware_boot::ram_min();
    let text_start = rust_hypervisor_firmware_boot::text_start();
    let text_end = rust_hypervisor_firmware_boot::text_end();
    let stack_start = rust_hypervisor_firmware_boot::stack_start();

    info!("RAM_MIN: {:#X}", ram_min);
    info!("TEXT_START: {:#X}", text_start);
    info!("TEXT_END: {:#X}", text_end);
    info!("STACK_START: {:#X}", stack_start);

    let stack_start = stack_start - 0xFFFFFFFF80000000;

    // Ensure we don't clash with existing structures.
    let mut start = range.start.start_address().as_u64() as usize;
    let limit = (range.end + 1).start_address().as_u64() as usize;

    if start < stack_start {
        start = stack_start;
    }

    info!("Using [{:#016x}..{:#016x}) for heap.", start, limit);
    // This is safe as we know the memory is available based on the e820 map.
    unsafe {
        ALLOCATOR.lock().init(start as *mut u8, limit - start);
    }
    Ok(())
}