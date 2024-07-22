use std::sync::{Arc, Mutex};
use crate::public_api::server;
mod cache;
mod input;
mod public_api;
mod config;

use crate::config::Settings;
use crate::input::handle_input;

use cache::Cache;

fn main(){
    //reading port from config.json
    let settings = Settings::new();
    let cache = Arc::new(Mutex::new(Cache::new(settings.port))); 
    let cache_clone = Arc::clone(&cache);
        std::thread::spawn(move || {
            actix_web::rt::System::new().block_on(async move {
                let settings:Settings = Settings::new();
                server::run_server(cache_clone,settings.port.to_string()).await.unwrap();
                println!("running");
            });
        });
    handle_input(cache) ;
}

