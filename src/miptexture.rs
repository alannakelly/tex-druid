use byteorder::{ReadBytesExt, NativeEndian};
use std::io::{SeekFrom, Seek, Read};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::str;
use crate::palette::Palette;

pub struct MipTexture {
    name: [u8; 16],
    pub width: u32,
    pub height: u32,
    offset1: u32,
    offset2: u32,
    offset4: u32,
    offset8: u32,
    image1: Vec<u8>,
    image2: Vec<u8>,
    image4: Vec<u8>,
    image8: Vec<u8>,
}

impl MipTexture {
    pub fn read(mut file: &File, offset: u64) -> MipTexture {
        let save_offset = file.seek(SeekFrom::Current(0)).unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        let mut t = MipTexture {
            name: [0; 16],
            width: 0,
            height: 0,
            offset1: 0,
            offset2: 0,
            offset4: 0,
            offset8: 0,
            image1: Vec::new(),
            image2: Vec::new(),
            image4: Vec::new(),
            image8: Vec::new(),
        };

        let mut name: [u8; 16] = [0; 16];
        file.read_exact(&mut name).expect("Error reading entry name.");
        for i in 0..(name.iter().position(|&c| c == 0)).unwrap() {
            t.name[i] = name[i];
        }
        t.width = file.read_u32::<NativeEndian>().unwrap();
        t.height = file.read_u32::<NativeEndian>().unwrap();
        t.offset1 = file.read_u32::<NativeEndian>().unwrap();
        t.offset2 = file.read_u32::<NativeEndian>().unwrap();
        t.offset4 = file.read_u32::<NativeEndian>().unwrap();
        t.offset8 = file.read_u32::<NativeEndian>().unwrap();

        let texSize = (t.width * t.height) as usize;
        t.image1.resize(texSize, 0);
        t.image2.resize(texSize / 2, 0);
        t.image4.resize(texSize / 4, 0);
        t.image8.resize(texSize / 8, 0);

        file.seek(SeekFrom::Start(offset + (t.offset1 as u64)));
        file.read_exact(t.image1.as_mut_slice());

        file.seek(SeekFrom::Start(save_offset)).unwrap();
        return t;
    }

    pub fn to_rgb_image(&self, pal: &Palette) -> Vec<u8> {
        let mut image = vec![0u8; self.image1.len() * 3];
        let mut i = 0;
        for p in self.image1.iter() {
            let rgb = pal.get_rgb(*p);
            image[i] = rgb.r;
            image[i + 1] = rgb.g;
            image[i + 2] = rgb.b;
            i += 3;
        }
        return image;
    }

    pub fn name(&self) -> &str {
        let name = str::from_utf8(&self.name).unwrap();
        return name.trim_matches(char::from(0));
    }
}

impl fmt::Display for MipTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name: {} width: {} height: {}", str::from_utf8(&self.name).unwrap(), self.width, self.height)
    }
}
