/* Copyright 2020 Alanna Kelly
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
  */
use crate::palette::Palette;
use std::fs::File;
use std::io::{BufWriter, Read};
use png::{ColorType, BitDepth};
use structview::{i32_le, i16_le, u32_le, View};
use std::{fmt, fs, mem};
use std::fmt::Formatter;
use std::path::Path;
use rgb::{RGB8, ComponentBytes};

pub(crate) fn write_png(file_path: &String, width: u32, height: u32, data: &[u8]) {
    let path = std::path::Path::new(&file_path);
    let file = File::create(path).unwrap();
    let ref mut bufw = BufWriter::new(file);
    let mut encoder = png::Encoder::new(bufw, width, height);
    encoder.set_color(ColorType::RGB);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

pub fn get_entry_name(bad: &[u8; 16]) -> String {
    let mut good = Vec::with_capacity(16);
    for c in bad.iter() {
        if *c == 0 { break; }
        good.push(*c);
    }
    return String::from_utf8(good).unwrap();
}

pub fn indexed_to_rgb(image: &[u8], palette: &Palette) -> Vec::<RGB8> {
    let mut rgb_image = Vec::<RGB8>::with_capacity(image.len());
    for pixel in image {
        rgb_image.push(palette.index(*pixel));
    }
    return rgb_image;
}

const ENTRY_PALETTE: i8 = 0x40;
const ENTRY_STATUS_IMAGE: i8 = 0x42;
const ENTRY_MIP_TEXTURE: i8 = 0x44;
const ENTRY_CONSOLE_IMAGE: i8 = 0x45;

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct WadHeader {
    magic: i32_le,
    numentries: i32_le,
    diroffset: i32_le,
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
    name: [u8; 16],
}

impl fmt::Display for WadEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", get_entry_name(&self.name))
    }
}

#[derive(Clone, Copy, View)]
#[repr(C)]
pub struct MipTexture {
    name: [u8; 16],
    width: u32_le,
    height: u32_le,
    offset1: u32_le,
    offset2: u32_le,
    offset4: u32_le,
    offset8: u32_le,
}

impl fmt::Display for MipTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name: {} width: {} height: {}", get_entry_name(&self.name), self.width, self.height)
    }
}

#[derive(Clone)]
pub struct WadFile {
    data: Vec<u8>,
    num_entries: i32
}

impl WadFile {
    pub fn load(path: &Path) -> WadFile {
        let size_in_bytes = fs::metadata(path).unwrap().len();
        println!("data length {}.", size_in_bytes);
        let mut file = File::open(path).unwrap();
        let mut wad = WadFile {
            data: Vec::with_capacity(size_in_bytes as usize)
        };
        file.read_to_end(&mut wad.data);
        return wad;
    }

    pub fn get_palette(&self) -> Result<Option<Palette>, structview::Error> {
        let mut pal = None;
        let header = WadHeader::view(&self.data)?;
        let mut offset = header.diroffset.to_int() as usize;
        while offset < self.data.len() {
            let entry: &WadEntry = WadEntry::view(&self.data[offset..])?;
            match entry.entry_type {
                ENTRY_PALETTE => {
                    let start = entry.offset.to_int() as usize;
                    let end = (entry.offset.to_int() + 768) as usize;
                    pal = Some(Palette::from_data(&self.data[start..end]));
                    //write_png(&format!("{}.png", get_entry_name(&entry.name)), 16, 16, pal.unwrap().to_image());
                }
                _ => {}
            }
            offset += mem::size_of::<WadEntry>();
        }
        return Ok(pal);
    }

    fn enumerate_entries(&self) {
        let header = WadHeader::view(&self.data)?;
        let mut offset = header.diroffset.to_int() as usize;
        while offset < self.data.len() {
            let entry: &WadEntry = WadEntry::view(&self.data[offset..])?;
        }
    }

    pub fn dump_textures(&self, path: &Path) -> Result<(), structview::Error> {
        std::fs::create_dir_all(path);
        match self.get_palette().unwrap() {
            Some(pal) => {
                let header = WadHeader::view(&self.data)?;
                let mut offset = header.diroffset.to_int() as usize;
                while offset < self.data.len() {
                    let entry: &WadEntry = WadEntry::view(&self.data[offset..])?;
                    match entry.entry_type {
                        ENTRY_MIP_TEXTURE => {
                            let start = entry.offset.to_int() as usize;
                            let end = entry.offset.to_int() as usize + mem::size_of::<MipTexture>();
                            let mip_texture: &MipTexture = MipTexture::view(&self.data[start..end])?;
                            let image_start = start + mip_texture.offset1.to_int() as usize;
                            let image_size = (mip_texture.width.to_int() * mip_texture.height.to_int()) as usize;
                            let image_end = image_start + image_size;
                            let image = &self.data[image_start..image_end];
                            let mut rgb_image = Vec::<RGB8>::with_capacity(image_size);
                            for pixel in image {
                                rgb_image.push(pal.index(*pixel));
                            }
                            let image_file_name = str::replace(&get_entry_name(&mip_texture.name)[..], "*", "#");
                            write_png(&format!("{}/{}.png", path.to_str().unwrap(), image_file_name), mip_texture.width.to_int(), mip_texture.height.to_int(), rgb_image.as_bytes());
                            println!("{}", mip_texture);
                            //break;
                        }
                        _ => {}
                    }
                    offset += mem::size_of::<WadEntry>();
                }
            }
            None => {
                ()
            }
        }
        Ok(())
    }
}

