
use std::ffi::OsString;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use crate::cache::Cache;

use winapi::shared::lmcons::NET_API_STATUS;
use windows_service::service::{
    ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode,
    ServiceInfo, ServiceStartType, ServiceState, ServiceType,
};

use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::service_control_handler::{
    self, ServiceControlHandlerResult
};
use winapi::um::lmaccess::{USER_INFO_1,NetUserAdd,UF_SCRIPT};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

pub fn winstr(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub fn set_user(user_name:&str,password:&str)-> u32 {
    let mut username = winstr(user_name);
    let mut password = winstr(password);
    let mut user = USER_INFO_1{
    usri1_name: username.as_mut_ptr(),
    usri1_password: password.as_mut_ptr(),
    usri1_priv: 1,
    usri1_password_age: 0,
    usri1_home_dir: std::ptr::null_mut(),
    usri1_comment: std::ptr::null_mut(),
    usri1_flags: UF_SCRIPT,
    usri1_script_path: std::ptr::null_mut(),
    };
    let mut error = 0 ;
    unsafe{
       let add_user_result =  NetUserAdd(std::ptr::null_mut(),1,&mut user as *mut _ as _,&mut error);
    };
    return error;
}

const SERVICE_NAME: &str = "RusselCacheService";
const SERVICE_DISPLAY_NAME: &str = "Russel Cache Service";
const SERVICE_DESCRIPTION: &str = "A service for managing Russel Cache";

fn install_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_binary_path = std::env::current_exe().unwrap();
    let password: Option<OsString> = Some(OsString::from("QAZWSXEDCRFVTGBYHNUJMIK!@^%$#"));
    let account_name: Option<OsString> = Some(OsString::from("RusselCacheServiceAccount"));
    let set_user_result =  set_user("RusselCacheServiceAccount", "QAZWSXEDCRFVTGBYHNUJMIK!@^%$#");
    println!("{:?}",set_user_result);
    let service_info = windows_service::service::ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: account_name,
        account_password: password,
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
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }

    };
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;
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

pub fn handle_input(_cache: Arc<Mutex<Cache>>) {
    loop {
        print_prompt();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts.as_slice() {
            ["--", "install_service"] => {
                match install_service() {
                    Ok(_) => println!("Service installed successfully."),
                    Err(err) => eprintln!("Failed to install service: {:?}", err),
                }
            }
            ["--", "start_service"] => {
                match start_service() {
                    Ok(_) => println!("Service run successfully."),
                    Err(err) => eprintln!("Failed to start service: {:?}", err),
                }
            }
            ["--", "stop_service"] => {
                match stop_service() {
                    Ok(_) => println!("Service stopped successfully."),
                    Err(err) => eprintln!("Failed to stop service: {:?}", err),
                }
            }
            ["--", "delete_service"] => {
                match delete_service() {
                    Ok(_) => println!("Service deleted successfully."),
                    Err(err) => eprintln!("Failed to delete service: {:?}", err),
                }
            }
            _ => println!("Invalid command. Use '-- install_service', '-- start_service', '-- stop_service', or '-- delete_service'."),
        }
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}