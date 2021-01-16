/* Copyright 2020 Alanna Kelly
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
  */
use std::env;
use std::path::Path;
use crate::wad2::{WadFile, MipTexture};
use druid::{WindowDesc, Widget, AppLauncher, WidgetExt, UnitPoint, Data, Lens, Env};
use druid::widget::{Label, Flex};
use std::sync::Arc;
use std::ops::Deref;

mod wad2;
mod palette;

#[derive(Clone, Data, Lens)]
struct AppData {
    path: String
}

/*
#[derive(Clone, Data, Lens)]
struct TextureItem {
    texture: MipTexture
}*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let wad_file : wad2::WadFile::load(Path::new(path.as_str()));
    let app_data = AppData {
        path
    };

    /*let filename = &args[1];
    println!("Texture Druid - Processing {}", filename);
    wad2::WadFile::load(Path::new(&args[1])).dump_textures(Path::new("./textures"));*/

    let data = wad_file.unwrap().get_palette().unwrap().unwrap().to_image();
    wad2::write_png(&format!("{}.png", "palette"), 16, 16, data);

    let main_window = WindowDesc::new(build_root_widget)
        .title("Texture Druid")
        .window_size((980.0, 540.0));
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(app_data)
        .expect("Failed to launch Application.");
}

fn build_root_widget() -> impl Widget<AppData> {
    let lblFilename = Label::new(|data: &AppData, _env: &Env| {
        data.path.to_string()
    }).with_text_size(32.0);

    Flex::column()
        .with_child(lblFilename)
        .align_vertical(UnitPoint::CENTER)
}