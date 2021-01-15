mod palette;
mod wad2;
mod miptexture;

use std::fmt;
use std::fmt::Formatter;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{SeekFrom, BufWriter};
use std::str;
use byteorder::{NativeEndian, ReadBytesExt};
use std::default;
use png::{ColorType, BitDepth};
use palette::Palette;
use wad2::EntryType;
use wad2::WadHeader;
use wad2::WadEntry;
use miptexture::MipTexture;
use std::path::Path;

/*typedef struct                 // Mip texture list header
{ long numtex;                 // Number of textures in Mip Texture list
  long offset[numtex];         // Offset to each of the individual texture
} mipheader_t;                 //  from the beginning of mipheader_t

Each individual texture is also a structured entry, that indicates the characteristics of the textures, and a pointer to scaled down picture data.

typedef struct                 // Mip Texture
{ char   name[16];             // Name of the texture.
  u_long width;                // width of picture, must be a multiple of 8
  u_long height;               // height of picture, must be a multiple of 8
  u_long offset1;              // offset to u_char Pix[width   * height]
  u_long offset2;              // offset to u_char Pix[width/2 * height/2]
  u_long offset4;              // offset to u_char Pix[width/4 * height/4]
  u_long offset8;              // offset to u_char Pix[width/8 * height/8]
} miptex_t;
*/

fn write_png(file_path:&str, width:u32, height:u32, data:&[u8]) {
    let path = std::path::Path::new(&file_path);
    let file = File::create(path).unwrap();
    let ref mut bufw = BufWriter::new(file);
    let mut encoder = png::Encoder::new(bufw,width,height);
    encoder.set_color(ColorType::RGB);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

fn main() {
    wad2::WadFile::load(Path::new("q.wad"));
    //let mut wad_file = fs::read("q.wad").expect("Unable to read file.");
    /*let mut pal = Palette::default();
    let mut file = File::open("q.wad").expect("unable to read fiel");
    let header = WadHeader::read(&file);
    assert_eq!(header.magic, 0x32444157);
    //assert_eq!(header.numentries, )
    println!("{}", header.numentries);
    println!("{}", header.diroffset);
    file.seek(SeekFrom::Start(header.diroffset as u64)).expect("Seek failed");
    for _ in 0..header.numentries {
        let entry = WadEntry::read(&file);
        if entry.entry_type == (EntryType::Palette as u8) {
            // print!("Palette found at {}.", entry.offset);
            pal = Palette::read(&file, entry.offset as u64);
            write_png(r"pal.png",16,16,&pal.to_image());
            // for e in pal.entries {
            //     println!("{}",e);
            // }
        } else if entry.entry_type == (EntryType::MipTexture as u8) {
            //println!("Texture {} found at {}.", str::from_utf8(&entry.name).unwrap(), entry.offset);
            let tex = MipTexture::read(&file, entry.offset as u64);
            println!("{}",tex);

            //write_png(&*format!("{}.png", tex.name()), tex.width, tex.height, &*tex.to_rgb_image(&pal));

        }
    }*/

}