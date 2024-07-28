use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the path to the source file
    let src = "./config/config.json"; // Change this to your source file

    // Get the path to the build directory
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{:?}",out_dir);

    let mut dest = PathBuf::from(out_dir);
    dest.push(".."); // Go up one level from the build directory
    dest.push(".."); // Go up another level
    dest.push(".."); // Go up another level to get to the root of the project
    dest.push("config"); // Go up another level to get to the root of the project
    
    // Create the build directory if it doesn't exist
    fs::create_dir_all(&dest).unwrap();

    // Append the filename to the destination path
    dest.push("config.json"); // Change this to your source file

    println!("{:?}",src);
    println!("{:?}",dest);
    // Copy the file to the build directory
    fs::copy(src, &dest).unwrap();

    println!("cargo:rerun-if-changed={}", src);
}