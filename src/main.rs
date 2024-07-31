use std::sync::{Arc, Mutex};
use std::thread;
use crate::public_api::server;
use local_ip_address::local_ip;
use memory_handling::memory_handling::MemoryHandler;
mod cache;
mod input;
mod public_api;
mod config;
mod memory_handling;
mod env_var;
use std::env;
use crate::config::Settings;
use crate::input::handle_input;
use cache::Cache;

fn main() {
    
    println!("|============================================================================|");
    println!("||  //////////  //     //   /////////   /////////    /////////    //        ||");
    println!("||  //      //  //     //   //     /    //     /     //           //        ||");
    println!("||  //     //   //     //   //          //           //           //        ||");
    println!("||  ////////    //     //   /////////   /////////    /////////    //        ||");
    println!("||  //     //   //     //          //          //    //           //        ||");
    println!("||  //     //   //     //   /     //    /     //     //           //        ||");
    println!("||  //     //   ////////   /////////   /////////     /////////    ///////// ||");
    println!("|============================================================================|");
    print!("
    ");
    
    let args: Vec<String> = env::args().collect();
    let main_args = args.last().unwrap();

    // Reading configurations from config.json
    let settings: Settings = Settings::new();
    let memory_handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let cache = Arc::new(Mutex::new(Cache::new(settings.port, memory_handler.clone())));
    let local_sys_ip = local_ip().unwrap();
    let cache_clone = Arc::clone(&cache);
    
    std::thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            let settings: Settings = Settings::new();
            server::run_server(cache_clone, settings.port.to_string(), "127.0.0.1".to_string()).await.unwrap();
        });
    });
    
    handle_input(cache, main_args.to_string());

}