
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;
use std::io::{SeekFrom, BufWriter};
use std::str;
use byteorder::{NativeEndian, ReadBytesExt};
use std::default;
use png::{ColorType, BitDepth};

enum EntryType {
    Palette = 0x40,
    StatusImage = 0x42,
    MipTexture = 0x44,
    ConsoleImage = 0x45
}

#[derive(Clone, Debug)]
struct RGB {
    r: u8,
    g: u8,
    b: u8
}

impl Default for RGB{
    fn default() -> RGB {
        RGB {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

struct Palette {
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
    fn read(mut file: &File, offset:u64) -> Palette {
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

    fn to_image(&self) -> [u8; 768] {
        let mut img: [u8; 768] = [0;768];
        let mut i = 0;
        for entry in self.entries.iter() {
            img[i] = entry.r;
            img[i+1] = entry.g;
            img[i+2] = entry.b;
            i+=3;
        }
        return img;
    }
}
impl fmt::Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"RGB({},{},{})", self.r, self.g, self.b)
    }
}

struct WadHeader {
    magic: u32,
    numentries: u32,
    diroffset: u32
}

impl WadHeader {
    fn read(mut file: &File) -> WadHeader {
        WadHeader {
            magic: file.read_u32::<NativeEndian>().unwrap(),
            numentries: file.read_u32::<NativeEndian>().unwrap(),
            diroffset: file.read_u32::<NativeEndian>().unwrap()
        }
    }
}

struct WadEntry {
    offset: u32,
    dsize: u32,
    size: u32,
    entry_type: u8,
    compression: u8,
    dummy: u16,
    name: [u8; 16]
}

impl WadEntry {
    fn read(mut file: &File) -> WadEntry {
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

struct MipTexture {
    name: [u8; 16],
    width: u32,
    height: u32,
    offset1: u32,
    offset2: u32,
    offset4: u32,
    offset8: u32
}

impl MipTexture {
    fn read(mut file: &File, offset: u64) -> MipTexture {
        let save_offset = file.seek(SeekFrom::Current(0)).unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        let mut t = MipTexture{
            name: [0;16],
            width: 0,
            height: 0,
            offset1: 0,
            offset2: 0,
            offset4: 0,
            offset8: 0
        };
        let mut name:[u8;16] = [0;16];
        file.read_exact(&mut name).expect("Error reading entry name.");
        for i in 0..(name.iter().position(|&c| c == 0)).unwrap() {
            t.name[i] = name[i];
        }
        t.width = file.read_u32::<NativeEndian>().unwrap();
        t.height = file.read_u32::<NativeEndian>().unwrap();
        t.offset1= file.read_u32::<NativeEndian>().unwrap();
        t.offset2 = file.read_u32::<NativeEndian>().unwrap();
        t.offset4 = file.read_u32::<NativeEndian>().unwrap();
        t.offset8 = file.read_u32::<NativeEndian>().unwrap();



        file.seek(SeekFrom::Start(save_offset)).unwrap();
        return t;
    }

}

impl fmt::Display for MipTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"name: {} width: {} height: {}", str::from_utf8(&self.name).unwrap(), self.width, self.height)
    }
}

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
    //let mut wad_file = fs::read("q.wad").expect("Unable to read file.");
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
            let pal = Palette::read(&file, entry.offset as u64);
            write_png(r"pal.png",16,16,&pal.to_image());
            // for e in pal.entries {
            //     println!("{}",e);
            // }
        } else if entry.entry_type == (EntryType::MipTexture as u8) {
            //println!("Texture {} found at {}.", str::from_utf8(&entry.name).unwrap(), entry.offset);
            let tex = MipTexture::read(&file, entry.offset as u64);
            println!("{}",tex);
        }
    }

}