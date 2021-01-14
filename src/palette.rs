use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use byteorder::ReadBytesExt;

#[derive(Debug, Clone, Copy)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for RGB {
    fn default() -> RGB {
        RGB {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

pub struct Palette {
    entries: Vec<RGB>
}

impl Default for Palette {
    fn default() -> Palette {
        Palette {
            entries: vec![RGB::default(); 256]
        }
    }
}

impl Palette {
    pub fn read(mut file: &File, offset: u64) -> Palette {
        let save_offset = file.seek(SeekFrom::Current(0)).unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        let mut pal = Palette::default();
        for entry in pal.entries.iter_mut() {
            entry.r = file.read_u8().unwrap();
            entry.g = file.read_u8().unwrap();
            entry.b = file.read_u8().unwrap();
        }
        file.seek(SeekFrom::Start(save_offset)).unwrap();
        return pal;
    }

    pub fn to_image(&self) -> [u8; 768] {
        let mut img: [u8; 768] = [0; 768];
        let mut i = 0;
        for entry in self.entries.iter() {
            img[i] = entry.r;
            img[i + 1] = entry.g;
            img[i + 2] = entry.b;
            i += 3;
        }
        return img;
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({},{},{})", self.r, self.g, self.b)
    }
}
