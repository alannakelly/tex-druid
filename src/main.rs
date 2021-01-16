/* Copyright 2020 Alanna Kelly
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
  */
use std::env;
use std::path::Path;

mod wad2;
mod palette;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Texture Druid - Processing {}", filename);
    wad2::WadFile::load(Path::new(&args[1])).dump_textures(Path::new("./textures"));
}