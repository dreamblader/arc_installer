use std::fs;
use std::io::Read;
use std::io::Bytes;
use std::fs::File;
use sha2::{Sha256, Digest};
use crate::loader::source_repos::ALL_SOURCES;


pub fn load_sources() {
    for plugin in &ALL_SOURCES {
        let dir_path =  format!("plugins/{}", plugin.name);
        let file_path = format!("{}/{}", dir_path, plugin.file_name);
        fs::create_dir_all(dir_path).expect("failed to create dir");
        let mut response = reqwest::blocking::get(plugin.url).expect("failed to load url");
        let mut plugin_file = File::create(&file_path).expect("failed to create plugin file");
        std::io::copy(&mut response, &mut plugin_file).expect("failed to copy response content");
        
    }
    
    //let mut out = File::create("rustup-init.sh").expect("failed to create file");
    //;
}