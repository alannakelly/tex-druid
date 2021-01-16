/* Copyright 2020 Alanna Kelly
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
  */
use rgb::{RGB8, FromSlice, ComponentBytes};
use std::convert::TryFrom;

pub struct Palette {
    entries: [RGB8;256]
}

impl Palette {
    pub fn from_data(data: &[u8]) -> Palette {
        Palette {
            entries: <[RGB8; 256]>::try_from(data.as_rgb()).unwrap()
        }
    }

    pub fn to_image(&self) -> &[u8] {
        return self.entries.as_bytes();
    }

    pub fn index(&self, index:u8) -> RGB8 {
        self.entries[index as usize]
    }
}