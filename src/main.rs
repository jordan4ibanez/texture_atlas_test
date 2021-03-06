
use core::num;
use std::{io::{BufReader, Read}, fs::{File, self, ReadDir}, path::Path, borrow::Borrow, str::from_utf8, ops::Add};



use image::{ImageBuffer, Rgba, DynamicImage, GenericImageView};

use mlua::{Lua, Table, FromLua};
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter,
    TexturePacker, TexturePackerConfig, texture,
};

pub fn with_path(path: &str) -> String {
    get_path_string() + path
}

pub fn load_resource(path: String) -> String {    
    let resource_result = fs::read_to_string(get_path_string() + path.as_str());    

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


// returns (number_of_textures, biggest_size_width, biggest_size_height)
fn configure_texture_atlas() -> (u32, u32, u32) {
    
    // size in pixels
    let mut biggest_size: (u32, u32) = (0,0);
    let mut number_of_textures: u32 = 0;

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

                                            // trim file name to .png
                                            let mut texture_file_name_mod = texture_file_name.clone();
                                            texture_file_name_mod.drain(0..length_of_name-4);
                                            
                                            if texture_file_name_mod.eq(".png") {

                                                number_of_textures += 1;

                                                let path = Path::new(texture_file_path_literal);
                                                let texture = ImageImporter::import_from_file(path).expect("UNABLE TO LOAD TEXTURE");

                                                // packer.pack_own(texture_file_name.to_string(), texture).unwrap();

                                                if texture.width() > biggest_size.0 {
                                                    biggest_size.0 = texture.width();
                                                }
                                                if texture.height() > biggest_size.1 {
                                                    biggest_size.1 = texture.height();
                                                }

                                                drop(texture);
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

    (number_of_textures, biggest_size.0, biggest_size.1)
}

fn load_block_texture(name: String, texture_name: String, mod_name: String, packer: &mut TexturePacker<DynamicImage, String>) {  
    
    let owned_path = with_path(&("/mods/".to_string() + &mod_name + "/textures/" + &texture_name)).clone();

    println!("{owned_path}");

    let path = Path::new(&owned_path);

    let texture = ImageImporter::import_from_file(path).expect("UNABLE TO LOAD TEXTURE");

    packer.pack_own(name, texture).unwrap();
}



fn load_lua_file(path: &str) -> String {
    let mut file: File = File::open(with_path(path)).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let text_str: &str = from_utf8(&buffer).unwrap();

    let test_string = text_str.clone().to_string().to_owned();

    test_string
}


fn main() {

    // (number of textures, biggest_width, biggest height)
    let config_values: (u32, u32, u32) = configure_texture_atlas();

    println!("{:?}", config_values);

    let configged_width: u32 = (config_values.0 + 2) / 2;
    let configged_height: u32 = (config_values.0 + 2) / 2;

    println!("{configged_width}");
    println!("{configged_height}");

    let config = TexturePackerConfig {
        max_width: config_values.1 * configged_width,
        max_height: config_values.2 * configged_height,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 0,
        texture_padding: 0,
        texture_extrusion: 0,
        trim: false,
    };
    
    
    let mut packer: TexturePacker<DynamicImage, String> = TexturePacker::new_skyline(config);



    // the debug "block component system"

    let mut id: Vec<u32> = Vec::new();
    let mut c_mod: Vec<String> = Vec::new();
    let mut name: Vec<String> = Vec::new();
    let mut texture: Vec<String> = Vec::new();



    let lua: Lua = Lua::new();

    lua.load(&load_lua_file("/context.lua")).exec().unwrap();

    let test: Table = lua.globals().raw_get("crafter").unwrap();

    let blocks: Table = test.get("blocks").unwrap();

    /*
    iterating -> crafter = {
        ....
    }
    */



            /*
            crafter = {
   iterating -> blocks = {

                }
            }
            */
    
    for blocks in blocks.pairs::<String, Table>() {

        let unwrapped_blocks: (String, Table);

        if blocks.is_ok() {
            unwrapped_blocks = blocks.unwrap();
        } else {
            panic!("ERROR INITIALIZING ROOT blocks TABLE!");
        }


        /*
        crafter = {
            blocks = {
    iterating -> dirt = {
                    def = value
                    next step
                    def = value
                }
            }
        }
        */

        // bool checks
        let name_checked = false;
        let texture_check = false;


        // push ID generically
        id.push((id.len() + 1) as u32);

        /*
        push mod to default as debug
        in final implementation this will track the mod name and implement it accordingly
        */

        // c_mod.push("default".to_string());

        println!("NEW BLOCK!");


        for definition in unwrapped_blocks.1.pairs::<String, String>() {
            

            let unwrapped_definition: (String, String) = definition.unwrap();

            // add to internal block component system with ID in final implementation

            println!("{}, {}", unwrapped_definition.0, unwrapped_definition.1);

            // this will need to push a result in final implementation

            match unwrapped_definition.0.as_str() {
                "name" => name.push(unwrapped_definition.1),
                "texture" => texture.push(unwrapped_definition.1),
                "mod" => c_mod.push(unwrapped_definition.1),
                _ => println!("SOMETHING HAS GONE WRONG!")                        
            }
        }
    }
        
    let mut iter_value = 0;

    for block_name in name.iter() {

        load_block_texture(block_name.to_string(), texture[iter_value].clone(), c_mod[iter_value].clone(), &mut packer);

        iter_value += 1;
    }
    
    let exporter = ImageExporter::export(&packer).unwrap();
        let mut file = File::create(with_path("/blah.png")).unwrap();
        exporter
            .write_to(&mut file, image::ImageFormat::Png)
            .unwrap();
}