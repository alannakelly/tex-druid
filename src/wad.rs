use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::str;

use byteorder::{NativeEndian, ReadBytesExt};

pub enum EntryType {
    Palette = 0x40,
    StatusImage = 0x42,
    MipTexture = 0x44,
    ConsoleImage = 0x45
}

pub struct WadHeader {
    pub magic: u32,
    pub numentries: u32,
    pub diroffset: u32
}

impl WadHeader {
    pub fn read(mut file: &File) -> WadHeader {
        WadHeader {
            magic: file.read_u32::<NativeEndian>().unwrap(),
            numentries: file.read_u32::<NativeEndian>().unwrap(),
            diroffset: file.read_u32::<NativeEndian>().unwrap()
        }
    }
}

pub struct WadEntry {
    pub offset: u32,
    dsize: u32,
    size: u32,
    pub entry_type: u8,
    compression: u8,
    dummy: u16,
    name: [u8; 16]
}

impl WadEntry {
    pub fn read(mut file: &File) -> WadEntry {
        let mut entry = WadEntry {
            offset: file.read_u32::<NativeEndian>().unwrap(),
            dsize: file.read_u32::<NativeEndian>().unwrap(),
            size: file.read_u32::<NativeEndian>().unwrap(),
            entry_type: file.read_u8().unwrap(),
            compression: file.read_u8().unwrap(),
            dummy: file.read_u16::<NativeEndian>().unwrap(),
            name: [0; 16]
        };

        let mut entry_name:[u8;16] = [0;16];
        file.read_exact(&mut entry_name).expect("Error reading entry name.");
        for i in 0..(entry_name.iter().position(|&c| c == 0)).unwrap() {
            entry.name[i] = entry_name[i];
        }

        return entry;
    }
}

impl fmt::Display for WadEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}", str::from_utf8(&self.name).unwrap())
    }
}
