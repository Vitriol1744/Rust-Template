/*
 * Created by v1tr10l7 on 10.02.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
use super::bitmap::Bitmap;
use limine::memory_map::EntryType;
use limine::request::MemoryMapRequest;

pub struct BitmapAllocator<'a> {
    page_bitmap: Bitmap<'a>,
    page_size: usize,
    total_memory: usize,
    used_memory: usize,
    usable_memory_top: usize,
    last_index: usize,
}

impl<'a> BitmapAllocator<'a> {
    pub fn initialize(
        &mut self,
        mmap_request: &MemoryMapRequest,
        page_size: usize,
        bitmap_ptr: *mut u8,
    ) -> Result<(), &'static str> {
        let response = mmap_request
            .get_response()
            .ok_or("Limine MMap request failed")?;
        let entries = response.entries();

        if !page_size.is_power_of_two() {
            return Err("Page size must be power of two");
        }

        self.page_size = page_size;
        let mut max_phys_addr = 0;

        for entry in entries {
            let top = (entry.base + entry.length) as usize;

            match entry.entry_type {
                EntryType::USABLE => {
                    self.usable_memory_top = core::cmp::max(self.usable_memory_top, top);
                    self.total_memory += entry.length as usize;
                }
                EntryType::ACPI_RECLAIMABLE
                | EntryType::BOOTLOADER_RECLAIMABLE
                | EntryType::EXECUTABLE_AND_MODULES => {
                    self.used_memory += entry.length as usize;
                    self.total_memory += entry.length as usize;
                }
                _ => {
                    self.total_memory += entry.length as usize;
                }
            }
            max_phys_addr = core::cmp::max(max_phys_addr, top);
        }

        let bitmap_entry_count = self.usable_memory_top / page_size;
        let bitmap_size_bytes = (bitmap_entry_count + 7) / 8;

        self.page_bitmap = Bitmap::from_raw(bitmap_ptr, bitmap_size_bytes);
        self.page_bitmap.set_all(0xff);

        for entry in entries {
            if entry.entry_type != EntryType::USABLE {
                continue;
            }

            let entry_base = entry.base as usize;
            let entry_len = entry.length as usize;

            let start_offset = if entry_base == 0 { 4096 } else { 0 };

            let mut offset = start_offset;
            while offset < entry_len {
                let phys_addr = entry_base + offset;
                self.page_bitmap.set(phys_addr / page_size, false); // false = free
                offset += page_size;
            }
        }

        Ok(())
    }
}
