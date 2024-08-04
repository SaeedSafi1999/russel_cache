use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::cache::Cache;
use crate::env_var::env_setter;
use crate::service_managment::windows_service_manager::{delete_service,install_service,stop_service,start_service};



pub fn handle_input(_cache: Arc<Mutex<Cache>>,mut args:String) {
   
        match args.as_str() {
            "set_variable" =>{
                let mut exe_dir:PathBuf = PathBuf::new();
                // Set environment variables
                if cfg!(debug_assertions) {
                 exe_dir = std::env::current_dir().unwrap().join("target").join("debug");  
                }
                else {
                 exe_dir = std::env::current_dir().unwrap(); 
                }
                 let env_setter_result =  env_setter::set_user_path_environment_variable(exe_dir.to_str().unwrap());
                 if env_setter_result.unwrap() == true{
                    println!("* Environment variable set successfully");
                 }
                 else{
                    println!("____Environment variable set before");
                 }
            }
            "install_service" => {
                match install_service() {
                    Ok(_) => println!("* Service installed successfully."),
                    Err(err) => eprintln!("____Failed to install service: {:?}", err),
                }
            }
            "start_service" =>{
                match start_service() {
                    Ok(_) => println!("* Service run successfully."),
                    Err(err) => eprintln!("___Failed to start service: {:?}", err),
                }
            }
            "stop_service" => {
                match stop_service() {
                    Ok(_) => println!("* Service stopped successfully."),
                    Err(err) => eprintln!("____Failed to stop service: {:?}", err),
                }
            }
            "delete_service" => {
                match delete_service() {
                    Ok(_) => println!("* Service deleted successfully. restart your system for apply cahnge"),
                    Err(err) => eprintln!("____Failed to delete service: {:?}", err),
                }
            }
            _ => println!("Invalid command. Use 'start_service' '-- install_service', '--stop_service','--set_variable' or '--delete_service'."),
        }
}


fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

