use std::fs;
use std::io::Read;
use std::io::Bytes;
use std::fs::File;
use sha2::{Sha256, Digest};
use crate::loader::plugin_repos::ALL_PLUGINS;


pub fn load_plugins() {
    for plugin in &ALL_PLUGINS {
        let dir_path =  format!("plugins/{}", plugin.name);
        let file_path = format!("{}/{}", dir_path, plugin.file_name);
        fs::create_dir_all(dir_path).expect("failed to create dir");
        let mut response = reqwest::blocking::get(plugin.url).expect("failed to load url");
        let mut plugin_file = File::create(&file_path).expect("failed to create plugin file");
        std::io::copy(&mut response, &mut plugin_file).expect("failed to copy response content");
        
        if is_hash_not_the_same(fs::read(&file_path).expect("failed to read plugin file"), file_path) {

        }
    }
    
    //let mut out = File::create("rustup-init.sh").expect("failed to create file");
    //;
}

fn is_hash_not_the_same(content_bytes:Vec<u8>, path:String) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(content_bytes);
    let final_hash = hasher.finalize();
    let mut hash_file = File::create(path +".sha256").expect("failed to create hash file");
    std::io::copy(&mut &final_hash[..], &mut hash_file).expect("failed to copy hash content");
    false
}