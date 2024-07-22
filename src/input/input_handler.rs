
use std::ffi::OsString;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::cache::Cache;
use crate::config::Settings;
use crate::public_api::server;

use windows_service::service::{
    ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode,
    ServiceInfo, ServiceStartType, ServiceState, ServiceType,
};

use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::service_control_handler::{
    self, ServiceControlHandlerResult
};

const SERVICE_NAME: &str = "RusselCacheService";
const SERVICE_DISPLAY_NAME: &str = "Russel Cache Service";
const SERVICE_DESCRIPTION: &str = "A service for managing Russel Cache";

fn install_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_binary_path = std::env::current_exe().unwrap();

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    service_manager.create_service(&service_info, ServiceAccess::START | ServiceAccess::STOP | ServiceAccess::DELETE)?;
    Ok(())
}


fn start_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = ServiceAccess::START;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    let args: Vec<OsString> = Vec::new();
    service.start(&args)?;
    Ok(())
}


fn stop_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = ServiceAccess::STOP;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    service.stop()?;
    Ok(())
}

fn delete_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = ServiceAccess::DELETE;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    service.delete()?;
    Ok(())
}

fn ffi_service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        eprintln!("Service error: {:?}", e);
        std::process::exit(1);
    }
}

fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    let status_handle = service_control_handler::register(SERVICE_NAME, move |control_event| {
        match control_event {
            ServiceControl::Stop => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    })?;

    let initial_status = windows_service::service::ServiceStatus {
        process_id: None,
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::from_secs(0),
    };
    status_handle.set_service_status(initial_status)?;

    loop {
        std::thread::sleep(Duration::from_secs(60));
    }
}

pub fn handle_input(cache: Arc<Mutex<Cache>>) {
    loop {
        print_prompt();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let parts: Vec<&str> = input.splitn(5, ' ').collect();
        if parts.is_empty() {
            continue;
        }

        
        
        match parts[0] {
            "russel" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "set" if parts.len() == 5 => { // set value for cluster
                            let cluster = parts[2].to_string();
                            let key = parts[3].to_string();
                            let value = parts[4].as_bytes().to_vec();
                            cache.lock().unwrap().set(cluster.clone(), key.clone(), value);
                            println!("Set [{}] {} = {}", cluster, key, parts[4]);
                        }
                        "set" if parts.len() == 3 => { // set cluster
                            let cluster = parts[2].to_string();
                            cache.lock().unwrap().set_cluster(cluster.clone());
                            println!("Cluster [{}] set", cluster);
                        }
                        "get_keys" if parts.len() == 3 => { // get keys of a cluster
                            let cluster = parts[2];
                            match cache.lock().unwrap().get_keys_of_cluster(cluster) {
                                Some(keys) => println!("Keys in cluster [{}]: {:?}", cluster, keys),
                                None => println!("Cluster [{}] not found", cluster),
                            }
                        }
                        "get" if parts.len() == 4 => { // get value
                            let cluster = parts[2];
                            let key = parts[3];
                            match cache.lock().unwrap().get(cluster, key) {
                                Some(value) => println!("{:?}", String::from_utf8_lossy(&value)),
                                None => println!("{} not found in cluster [{}]", key, cluster),
                            }
                        }
                        "delete" if parts.len() == 4 => { // delete value
                            let cluster = parts[2];
                            let key = parts[3];
                            cache.lock().unwrap().delete(cluster, key);
                            println!("Deleted {} from cluster [{}]", key, cluster);
                        }
                        "clear_cluster" if parts.len() == 3 => { // clear cluster
                            let cluster = parts[2];
                            cache.lock().unwrap().clear_cluster(cluster);
                            println!("Cleared cluster [{}]", cluster);
                        }
                        "clear_all" => { // clear all the cache
                            cache.lock().unwrap().clear_all();
                            println!("Cleared all clusters");
                        }
                        "get_clusters" => { // get clusters
                            let clusters = cache.lock().unwrap().get_all_clusters();
                            let port = cache.lock().unwrap().get_default_port();
                            println!("Clusters on port {} are: {:?}", port, clusters);
                        }
                        "port" => { // port that run on
                            let port = cache.lock().unwrap().get_default_port();
                            println!("Port is: {}", port);
                        }
                        "install_service" => {
                            match install_service() {
                                Ok(_) => println!("Service installed successfully."),
                                Err(err) => eprintln!("Failed to install service: {:?}", err),
                            }
                        }
                        "start_service" => {
                            match start_service() {
                                Ok(_) => println!("Service started successfully."),
                                Err(err) => eprintln!("Failed to start service: {:?}", err),
                            }
                        }
                        "stop_service" => {
                            match stop_service() {
                                Ok(_) => println!("Service stopped successfully."),
                                Err(err) => eprintln!("Failed to stop service: {:?}", err),
                            }
                        }
                        "delete_service" => {
                            match delete_service() {
                                Ok(_) => println!("Service deleted successfully."),
                                Err(err) => eprintln!("Failed to delete service: {:?}", err),
                            }
                        }
                        "help" => {
                            println!("for set use => russel set [cluster name] [key] [value]");
                            println!("for set cluster => russel set [cluster name]");
                            println!("for get use => russel get [cluster name] [key]");
                            println!("for delete use => russel delete [cluster name] [key]");
                            println!("for clear cluster => russel clear_cluster [cluster name]");
                            println!("for clear all => russel clear_all");
                            println!("for get clusters name => russel get_clusters");
                            println!("see port that app is running on => russel port");
                            println!("for install service => russel install_service");
                            println!("for start service => russel start_service");
                            println!("for stop service => russel stop_service");
                            println!("for delete service => russel delete_service");
                            println!("for kill process => russel exit");
                        }
                        //"exit" => break,
                        _ => println!("Invalid command. Use 'russel help' to see available commands."),
                    }
                }
            }
            _ => println!("Invalid command prefix. Use 'russel help' to start commands."),
        }
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}