use std::{fmt, fs};
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::str;

use byteorder::{NativeEndian, ReadBytesExt};
use std::path::Path;

pub enum EntryType {
    Palette = 0x40,
    StatusImage = 0x42,
    MipTexture = 0x44,
    ConsoleImage = 0x45
}

struct WadFile {
    data: Box<u8>
}

impl WadFile {
    pub fn load(path: &ath) -> WadFile {
        let size_in_bytes = fs::metadata(path)?.len();
        let file = File::open(path);
        file.seek(Seek::Start(0));

    }
}

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct WadHeader {
    pub magic: i32,
    pub numentries: i32,
    pub diroffset: i32
}

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct WadEntry {
    offset: i32,
    dsize: i32,
    size: i32,
    entry_type: i8,
    compression: i8,
    dummy: i16,
    name: [i8; 16]
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
