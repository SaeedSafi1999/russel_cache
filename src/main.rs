use std::sync::{Arc, Mutex};
use std::thread;
use crate::public_api::server;
use memory_handling::memory_handling::MemoryHandler;
mod cache;
mod input;
mod public_api;
mod config;
mod memory_handling;
mod env_var;
mod service_managment;
use std::env;
use crate::config::Settings;
use crate::input::handle_input;
use cache::Cache;

fn main() {
    println!("
    ");
    println!("|============================================================================|");
    println!("||  //////////  //     //   /////////   /////////    /////////    //        ||");
    println!("||  //      //  //     //   //     /    //     /     //           //        ||");
    println!("||  //     //   //     //   //          //           //           //        ||");
    println!("||  ////////    //     //   /////////   /////////    /////////    //        ||");
    println!("||  //     //   //     //          //          //    //           //        ||");
    println!("||  //     //   //     //   /     //    /     //     //           //        ||");
    println!("||  //     //   ////////   /////////   /////////     /////////    ///////// ||");
    println!("|============================================================================|");
    println!("
    ");
    
    let args: Vec<String> = env::args().collect();
    let main_args = args.last().unwrap();

    // Reading configurations from config.json
    let settings: Settings = Settings::new();
    let memory_handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let cache = Arc::new(Mutex::new(Cache::new(settings.port, memory_handler.clone())));
    let cache_clone = Arc::clone(&cache);
    
    std::thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            let settings: Settings = Settings::new();
            server::run_server(cache_clone, settings.port.to_string(), "127.0.0.1".to_string()).await.unwrap();
        });
    });
    println!("{:?}",main_args);
    if args.len() > 1{
        handle_input(cache, main_args.to_string());
    }
    loop {
        std::thread::park();
    }

}