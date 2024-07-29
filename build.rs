use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let src = "./config/config.json"; 

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{:?}",out_dir);

    let mut dest = PathBuf::from(out_dir);
    dest.push(".."); 
    dest.push(".."); 
    dest.push(".."); 
    dest.push("config"); 
    

    fs::create_dir_all(&dest).unwrap();


    dest.push("config.json"); 

    println!("{:?}",src);
    println!("{:?}",dest);
    fs::copy(src, &dest).unwrap();

    println!("cargo:rerun-if-changed={}", src);
}