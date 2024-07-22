use std::sync::{Arc, Mutex};

mod cache;
mod input;
mod public_api;
mod config;

use crate::config::Settings;
use crate::input::handle_input;

use cache::Cache;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let cache = Arc::new(Mutex::new(Cache::new())); // Wrap Cache in Arc<Mutex<...>>
//     server::run_server(cache).await
// }

fn main(){
    // let cache = cache::Cache::new(); 
    let settings = Settings::new();
    let cache = Arc::new(Mutex::new(Cache::new(settings.port))); 
    handle_input(cache) ;
}


// use std::sync::{Arc, Mutex};
// use windows_service::{
//     define_windows_service,
//     service::{
//         ServiceControl, ServiceControlHandler, ServiceControlHandlerResult, ServiceState, ServiceType,
//         ServiceStatus, ServiceExitCode,
//     },
//     service_control_handler::{self, ServiceControlHandlerResult},
//     service_dispatcher,
//     Result,
// };

// mod cache;
// mod input;
// mod public_api;

// use crate::input::handle_input;
// use crate::public_api::server;

// const SERVICE_NAME: &str = "YourServiceName";
// const SERVICE_DISPLAY_NAME: &str = "Your Service Display Name";
// const SERVICE_DESCRIPTION: &str = "Description of your service.";

// define_windows_service!(ffi_service_main, service_main);

// fn service_main(arguments: Vec<String>) {
//     if let Err(_e) = run_service(arguments) {
//         // Handle errors gracefully
//     }
// }

// fn run_service(_arguments: Vec<String>) -> Result<()> {
//     // Initialize your application components
//     let cache = Arc::new(Mutex::new(cache::Cache::new()));
//     let server_cache = cache.clone();
//     let input_cache = cache.clone();

//     // Spawn a thread for the server
//     let server_handle = std::thread::spawn(move || {
//         if let Err(e) = tokio::runtime::Runtime::new().unwrap().block_on(async {
//             if let Err(e) = server::run_server(server_cache.lock().unwrap().clone()).await {
//                 eprintln!("Server error: {}", e);
//             }
//         }) {
//             eprintln!("Error in server thread: {}", e);
//         }
//     });

//     // Handle input or other logic in the main thread
//     handle_input(&input_cache.lock().unwrap()); // Example of handling input

//     // Wait for the server thread to finish
//     if let Err(e) = server_handle.join() {
//         eprintln!("Error joining server thread: {:?}", e);
//     }

//     Ok(())
// }

// fn service_control_handler(control: u32) -> ServiceControlHandlerResult {
//     match control {
//         // Handle service control events here
//         _ => ServiceControlHandlerResult::NoError,
//     }
// }

// fn main() -> Result<()> {
//     // Register the service with Windows
//     service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;

//     // Initialize the service manager
//     let service_manager = windows_service::service_manager::ServiceManager::local_computer_service_manager()?;

//     // Create the service
//     service_manager.create_service(
//         SERVICE_NAME,
//         SERVICE_DISPLAY_NAME,
//         SERVICE_DESCRIPTION,
//         Some(std::env::current_exe().unwrap()),
//         ServiceType::OwnProcess,
//         ServiceStartType::AutoStart,
//         ServiceErrorControl::Normal,
//     )?;

//     // Start the service dispatcher
//     service_dispatcher::start(SERVICE_NAME, service_main)?;

//     Ok(())
// }
