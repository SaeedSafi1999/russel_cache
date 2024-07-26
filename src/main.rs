use std::sync::{Arc, Mutex};
use crate::public_api::server;
use local_ip_address::local_ip;
use memory_handling::memory_handling::MemoryHandler;
use crypto::crypto_service::VigenereCipher;
mod cache;
mod input;
mod public_api;
mod config;
mod memory_handling;
mod crypto;
mod env_var;

use crate::config::Settings;
use crate::input::handle_input;

use cache::Cache;

fn main(){
    //set env vars
    let exe_dir = std::env::current_dir().unwrap().join("target").join("debug");
    let env_setter_res = env_var::env_setter::set_user_environment_variable("RUSSEL_CACHE", exe_dir.to_str().unwrap());
    //reading configurations from config.json
    let settings: Settings = Settings::new();
    let mut memory_handler = Arc::new(Mutex::new(MemoryHandler::new()));
    let cache = Arc::new(Mutex::new(Cache::new(settings.port,memory_handler.clone()))); 
    let local_sys_ip = local_ip().unwrap();
    let cache_clone = Arc::clone(&cache);
        std::thread::spawn(move || {
            actix_web::rt::System::new().block_on(async move {
                let settings:Settings = Settings::new();
                server::run_server(cache_clone,settings.port.to_string(),local_sys_ip.to_string()).await.unwrap();
            });
        });
    handle_input(cache) ;
}

