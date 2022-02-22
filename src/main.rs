
use std::{io::BufReader, fs::{File, self, ReadDir}, path::Path, borrow::Borrow};

use image::{ImageBuffer, Rgba, DynamicImage};

use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter,
    TexturePacker, TexturePackerConfig, texture,
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


fn iterate_mod_textures(packer: &mut TexturePacker<DynamicImage, &str>){
    
    let mods: ReadDir = fs::read_dir(with_path("/mods/")).unwrap();

    // iterate mods directory
    for path in mods {
        match path {
            // found mod
            Ok(dir_entry) => {
                let mod_dir = fs::read_dir(dir_entry.path()).unwrap();
                // iterate mod directory
                for mod_path in mod_dir {
                    match mod_path {
                        Ok(mod_subdir) => {

                            // check if it has a textures folder
                            let mut test = mod_subdir.path().as_os_str().to_str().unwrap().to_string();
                            let length = test.len();
                            test.drain(0..length - 8);

                            // found textures
                            if test.eq("textures") {

                                let texture_files = fs::read_dir(mod_subdir.path()).unwrap();

                                // iterate files in the texture folder
                                for texture in texture_files {
                                    match texture {
                                        Ok(texture_file) => {

                                            // check if the file name ends with png
                                            let texture_file_name = &texture_file.file_name().as_os_str().to_str().unwrap().to_string().clone();

                                            // juggling the borrower
                                            let texture_file_path = &texture_file.path().clone().to_owned();
                                            let texture_file_path_literal = texture_file_path.as_os_str().to_str().unwrap();
                                            let length_of_name = &texture_file_name.len();

                                            let texture_file_clone = texture_file.file_name().clone().to_owned();

                                            // trim file name to .png
                                            let mut texture_file_name_mod = texture_file_name.clone();
                                            texture_file_name_mod.drain(0..length_of_name-4);
                                            
                                            if texture_file_name_mod.eq(".png") {


                                                // REALLY juggling the borrower
                                                let texture_file_literal = texture_file_path_literal;

                                                let file_name = texture_file_clone.as_os_str().to_owned();

                                                let path = Path::new(texture_file_literal);
                                                let texture = ImageImporter::import_from_file(path).expect("UNABLE TO LOAD TEXTURE");

                                                // this is horrible - a literal forced memory leak
                                                let my_string: &'static str = Box::leak(String::from(file_name.clone().to_str().unwrap().to_string()).into_boxed_str());

                                                packer.pack_own(my_string, texture).unwrap();
                                            }
                                        },
                                        Err(error) => {
                                            println!("{}", error);
                                            panic!("MOD TEXTURE DIRECTORY ERROR!");
                                        },
                                    }
                                }
                            }

                            //println!("{}", test);

                        },

                        Err(error) => {
                            println!("{}", error);
                            panic!("MOD DIRECTORY ERROR!");
                        },
                    }
                }
            },
            Err(error) => {
                println!("{}", error);
                panic!("MOD ROOT DIRECTORY ERROR!");
            }
        }
    }
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
    
    let mut packer: TexturePacker<DynamicImage, &str> = TexturePacker::new_skyline(config);

    iterate_mod_textures(&mut packer);


    /*
    let path_str = with_path("/texture/debug_alpha.png");
    let path = Path::new(path_str.as_str());
    let texture = ImageImporter::import_from_file(path).expect("unable to import file");

    packer.pack_own("debug_alpha", texture).unwrap();
    */

    

    let exporter = ImageExporter::export(&packer).unwrap();
        let mut file = File::create(with_path("/blah.png")).unwrap();
        exporter
            .write_to(&mut file, image::ImageFormat::Png)
            .unwrap();
}