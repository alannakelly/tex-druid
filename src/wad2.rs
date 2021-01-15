use std::{fmt, fs};
use std::fmt::{Formatter, Error};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;
use byteorder::{NativeEndian, ReadBytesExt};
use structview::{View, i32_le, i16_le};

pub enum EntryType {
    Palette = 0x40,
    StatusImage = 0x42,
    MipTexture = 0x44,
    ConsoleImage = 0x45
}

pub struct WadFile {
    data: Vec<u8>
}

impl WadFile {
    pub fn load(path: &Path) -> Result<(), structview::Error> {
        let size_in_bytes = fs::metadata(path).unwrap().len();
        println!("data length {}.", size_in_bytes);
        let mut file = File::open(path).unwrap();
        let mut wad = WadFile {
            data: Vec::with_capacity(size_in_bytes as usize)
        };
        file.read_to_end(&mut wad.data);
        println!("data length {}.", wad.data.len());
        let header = WadHeader::view(&wad.data)?;
        println!("directory offset {}.", header.diroffset);
        let mut offset = header.diroffset.to_int() as usize;
        while offset < wad.data.len() {
            let entry:&WadEntry = WadEntry::view(&wad.data[offset..])?;
            println!("{}", entry);
            offset += std::mem::size_of::<WadEntry>();
        }
        Ok(())
    }
}

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct WadHeader {
    magic: i32_le,
    numentries: i32_le,
    diroffset: i32_le
}

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct WadEntry {
    offset: i32_le,
    dsize: i32_le,
    size: i32_le,
    entry_type: i8,
    compression: i8,
    dummy: i16_le,
    name: [u8; 16]
}

fn fix_bad_entry_name(bad:&[u8; 16]) -> Vec<u8> {
    let mut good= Vec::with_capacity(16);
    for c in bad.iter() {
        if *c == 0 { break; }
        good.push(*c);
    }
    return good;
}

impl fmt::Display for WadEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}", str::from_utf8(&*fix_bad_entry_name(&self.name)).unwrap())
    }
}
