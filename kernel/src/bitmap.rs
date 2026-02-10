/*
 * Created by v1tr10l7 on 10.02.2026.
 * Copyright (c) 2024-2026, Szymon Zemke <v1tr10l7@proton.me>
 *
 * SPDX-License-Identifier: GPL-3
 */
pub struct Bitmap<'a> {
    data: &'a mut [u8],
    entry_count: usize,
}

impl<'a> Bitmap<'a> {
    pub fn from_raw(ptr: *mut u8, size_bytes: usize) -> Self {
        unsafe {
            Self {
                data: core::slice::from_raw_parts_mut(ptr, size_bytes),
                entry_count: size_bytes * 8,
            }
        }
    }

    pub fn get(&self, index: usize) -> bool {
        let byte_idx = index / 8;
        let bit_idx = index % 8;

        self.data
            .get(byte_idx)
            .map(|&byte| (byte & (1 << bit_idx)) != 0)
            .unwrap_or(false)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let byte_idx = index / 8;
        let bit_idx = index % 8;

        if let Some(byte) = self.data.get_mut(byte_idx) {
            if value {
                *byte |= 1 << bit_idx;
            } else {
                *byte &= !(1 << bit_idx);
            }
        }
    }

    pub fn set_all(&mut self, value: u8) {
        for byte in self.data.iter_mut() {
            *byte = value;
        }
    }

    pub fn find_first_not_set(&self, start: usize, end: usize) -> Option<usize> {
        for i in start..core::cmp::min(end, self.entry_count) {
            if !self.get(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn bit_count(&self) -> usize {
        self.entry_count
    }
}
