
use std::{io::BufReader, fs::{File, self}, path::Path};

use image::{ImageBuffer, Rgba};

use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter,
    TexturePacker, TexturePackerConfig,
};

pub fn with_path(path: &str) -> String {
    get_path_string() + path
}

pub fn load_resource(path: String) -> String {    
    let resource_result = fs::read_to_string(get_path_string() + &path);    

    match resource_result {
        Ok(data) => data,
        Err(_) => panic!("FAILED TO LOAD: {}!", &path[..]),
    }    
}

pub fn get_path_string() -> String {
    std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned()
}

// creates a usable image buffer in rgba 8 format
pub fn create_image_buffer(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let image: File = File::open(with_path(path)).expect(&("COULD NOT LOAD IMAGE IN ".to_string() + path));
    let buffered_reader: BufReader<File> = BufReader::new(image);
    image::load(buffered_reader, image::ImageFormat::Png).unwrap().to_rgba8()
}


fn main() {

    let config = TexturePackerConfig {
        max_width: 16*32,
        max_height: 16*32,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 0,
        texture_padding: 0,
        texture_extrusion: 0,
        trim: false,
    };
    
    let mut packer = TexturePacker::new_skyline(config);
    let path_str = with_path("/texture/debug_alpha.png");
    let path = Path::new(path_str.as_str());
    let texture = ImageImporter::import_from_file(path).expect("unable to import file");

    packer.pack_own("debug_alpha", texture).unwrap();

    let path_str = with_path("/texture/dirt.png");
    let path = Path::new(path_str.as_str());
    let texture = ImageImporter::import_from_file(path).expect("unable to import file");

    packer.pack_own("dirt", texture).unwrap();

    let exporter = ImageExporter::export(&packer).unwrap();
        let mut file = File::create(with_path("/blah.png")).unwrap();
        exporter
            .write_to(&mut file, image::ImageFormat::Png)
            .unwrap();
}