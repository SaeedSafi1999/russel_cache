
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use std::mem::zeroed;
use std::slice;
use std::iter::once;

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


const USER_PRIV_USER: u32 = 1;
const UF_SCRIPT: u32 = 0x0001;
const NERR_SUCCESS: u32 = 0;

#[repr(C)]
struct USER_INFO_1 {
    usri1_name: *mut u16,
    usri1_password: *mut u16,
    usri1_priv: u32,
    usri1_home_dir: *mut u16,
    usri1_comment: *mut u16,
    usri1_flags: u32,
    usri1_script_path: *mut u16,
    usri1_password_age: u32,
}

pub fn winstr(value: &str) -> Vec<u16> {
    std::ffi::OsStr::new(value).encode_wide().chain(once(0)).collect()
}

// Define NetUserAdd function
extern "system" {
    fn NetUserAdd(
        servername: *mut u16,
        level: u32,
        buf: *mut std::ffi::c_void,
        parm_err: *mut u32,
    ) -> u32;
}


pub fn set_user(user_name: &str, password: &str) -> u32 {
    let mut username = winstr(user_name);
    let mut password = winstr(password);

    let mut user = USER_INFO_1 {
        usri1_name: username.as_mut_ptr(),
        usri1_password: password.as_mut_ptr(),
        usri1_priv: USER_PRIV_USER,
        usri1_home_dir: null_mut(),
        usri1_comment: null_mut(),
        usri1_flags: UF_SCRIPT,
        usri1_script_path: null_mut(),
        usri1_password_age: 0,
    };

    let mut error = 0;
    unsafe {
        let add_user_result = NetUserAdd(
            null_mut(),
            1,
            &mut user as *mut _ as *mut std::ffi::c_void,
            &mut error,
        );
        if add_user_result != NERR_SUCCESS {
            eprintln!(
                "Failed to add user: error code {}, extended error code {}",
                add_user_result, error
            );
            return add_user_result;
        }
    }
    NERR_SUCCESS
}


const SERVICE_NAME: &str = "RusselCacheService";
const SERVICE_DISPLAY_NAME: &str = "Russel Cache Service";
const SERVICE_DESCRIPTION: &str = "A service for managing Russel Cache";

fn install_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_binary_path = std::env::current_exe().unwrap();
    // let password: Option<OsString> = Some(OsString::from("QAZWSXEDCRFVTGBYHNUJMIK!@^%$#"));
    // let account_name: Option<OsString> = Some(OsString::from("RusselCache"));
    // let set_user_result =  set_user("RusselCacheServiceAccount", "QAZWSXEDCRFVTGBYHNUJMIK!@^%$#");
    // if set_user_result == NERR_SUCCESS {
    //     println!("User added successfully");
    // } else {
    //     println!("Failed to add user, error code: {}", set_user_result);
    // }
    let service_info = windows_service::service::ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Ignore,
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