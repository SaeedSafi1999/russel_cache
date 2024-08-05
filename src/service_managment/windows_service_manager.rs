// extern crate winapi;

// use std::ffi::OsString;
// use std::net::{IpAddr, SocketAddr, UdpSocket};
// use std::os::windows::ffi::OsStrExt;
// use std::ptr::null_mut;
// use std::sync::mpsc;
// use std::sync::atomic::{AtomicBool, Ordering};
// use winapi::um::winsvc::{
//     RegisterServiceCtrlHandlerExW, SetServiceStatus, StartServiceCtrlDispatcherW, SERVICE_ACCEPT_STOP, SERVICE_ACTIVE, SERVICE_CONTROL_INTERROGATE, SERVICE_CONTROL_PAUSE, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STATUS, SERVICE_STATUS_HANDLE, SERVICE_STOPPED, SERVICE_TABLE_ENTRYW
// };
// use winapi::shared::minwindef::{DWORD, LPVOID, TRUE};
// use winapi::um::errhandlingapi::GetLastError;
// use winapi::shared::winerror::ERROR_CALL_NOT_IMPLEMENTED;
// const SERVICE_WIN32_OWN_PROCESS: DWORD = 0x00000010;
// const SERVICE_NAME: &str = "Rusel_Cache_Service6";
// const SERVICE_DISPLAY_NAME: &str = "russel cache services";
// const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
// const SENDER_PORT: u16 = 1234;
// const RECEIVER_PORT: u16 = 8080;
// const PING_MESSAGE: &str = "ping";
// static SERVICE_RUNNING_FLAG: AtomicBool = AtomicBool::new(false);

// pub fn install_service() -> windows_service::Result<()> {
//     use windows_service::{
//         service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
//         service_manager::{ServiceManager, ServiceManagerAccess},
//     };

//     let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
//     let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
//     let service_binary_path = ::std::env::current_exe().unwrap();
//     println!("{:?}",service_binary_path);
//     let service_info = ServiceInfo {
//         name: OsString::from(SERVICE_NAME),
//         display_name: OsString::from(SERVICE_DISPLAY_NAME),
//         service_type: ServiceType::OWN_PROCESS,
//         start_type: ServiceStartType::AutoStart,
//         error_control: ServiceErrorControl::Normal,
//         executable_path: service_binary_path,
//         launch_arguments: vec![],
//         dependencies: vec![],
//         account_name: None,
//         account_password: None,
//     };

//     let service = service_manager.create_service(&service_info, ServiceAccess::START | ServiceAccess::STOP | ServiceAccess::CHANGE_CONFIG | ServiceAccess::PAUSE_CONTINUE)?;
//     service.set_description("This service runs Russel cache and checks health of the application")?;
    
//     Ok(())
// }

// pub fn start_service() -> windows_service::Result<()> {
//     run();
//     Ok(())
// }

// pub fn stop_service() -> windows_service::Result<()> {
//     let manager_access = windows_service::service_manager::ServiceManagerAccess::CONNECT;
//     let service_manager = windows_service::service_manager::ServiceManager::local_computer(None::<&str>, manager_access)?;
//     let service_access = windows_service::service::ServiceAccess::STOP;
//     let service = service_manager.open_service(SERVICE_NAME, service_access)?;
//     service.stop()?;
//     Ok(())
// }

// pub fn delete_service() -> windows_service::Result<()> {
//     let manager_access = windows_service::service_manager::ServiceManagerAccess::CONNECT;
//     let service_manager = windows_service::service_manager::ServiceManager::local_computer(None::<&str>, manager_access)?;
//     let service_access = windows_service::service::ServiceAccess::DELETE;
//     let service = service_manager.open_service(SERVICE_NAME, service_access)?;
//     service.delete()?;
//     Ok(())
// }

// fn run() -> Result<(), u32> {
//     let service_name: Vec<u16> = OsString::from(SERVICE_NAME).encode_wide().chain(Some(0)).collect();
//     let service_table: [SERVICE_TABLE_ENTRYW; 2] = [
//         SERVICE_TABLE_ENTRYW {
//             lpServiceName: service_name.as_ptr(),
//             lpServiceProc: Some(service_main),
//         },
//         SERVICE_TABLE_ENTRYW {
//             lpServiceName: null_mut(),
//             lpServiceProc: None,
//         },
//     ];

//     let success = unsafe { StartServiceCtrlDispatcherW(service_table.as_ptr()) };
//     if success == TRUE {
//         Ok(())
//     } else {
//         Err(unsafe { GetLastError() })
//     }
// }

// // Service main function
// unsafe extern "system" fn service_main(_argc: DWORD, _argv: *mut *mut u16) {
//     // Register the service control handler
//     let service_name: Vec<u16> = OsString::from(SERVICE_NAME).encode_wide().chain(Some(0)).collect();
//     let service_status_handle = RegisterServiceCtrlHandlerExW(
//         service_name.as_ptr(),
//         Some(service_control_handler),
//         null_mut(),
//     );

//     if service_status_handle.is_null() {
//         return;
//     }

//     // Set the service status to start pending
//     let mut service_status = SERVICE_STATUS {
//         dwServiceType: SERVICE_WIN32_OWN_PROCESS,
//         dwCurrentState: SERVICE_START_PENDING,
//         dwControlsAccepted: 0,
//         dwWin32ExitCode: 0,
//         dwServiceSpecificExitCode: 0,
//         dwCheckPoint: 0,
//         dwWaitHint: 30000,
//     };

//     SetServiceStatus(service_status_handle, &mut service_status);

//     // Set the service status to running
//     service_status.dwCurrentState = SERVICE_RUNNING;
//     service_status.dwControlsAccepted = SERVICE_ACCEPT_STOP;
//     service_status.dwWaitHint = 0;
//     SetServiceStatus(service_status_handle, &mut service_status);

//     SERVICE_RUNNING_FLAG.store(true, Ordering::SeqCst);

//     // Run the service
//     if let Err(e) = run_service(service_status_handle) {
//         eprintln!("Service encountered an error: {}", e);
//     }

//     // Set the service status to stopped
//     service_status.dwCurrentState = SERVICE_STOPPED;
//     service_status.dwControlsAccepted = 0;
//     SetServiceStatus(service_status_handle, &mut service_status);
// }

// // Service control handler
// unsafe extern "system" fn service_control_handler(
//     control: DWORD,
//     _event_type: DWORD,
//     _event_data: LPVOID,
//     _context: LPVOID,
// ) -> DWORD {
//     match control {
//         SERVICE_CONTROL_STOP => {
//             SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
//             return winapi::shared::winerror::NOERROR as DWORD;
//         }
//         SERVICE_ACTIVE => {
//             SERVICE_RUNNING_FLAG.store(true, Ordering::SeqCst);
//             return winapi::shared::winerror::NOERROR as DWORD;
//         }
//         SERVICE_CONTROL_STOP => {
//             return winapi::shared::winerror::NOERROR as DWORD;
            
//         }
//         SERVICE_CONTROL_PAUSE => {
//             return winapi::shared::winerror::NOERROR as DWORD;
//         }
//         SERVICE_CONTROL_INTERROGATE => {
//             return winapi::shared::winerror::NOERROR as DWORD;
//         }
//         SERVICE_CONTROL_SHUTDOWN => {
//             SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
//             return winapi::shared::winerror::NOERROR as DWORD;
//         }
//         SERVICE_RUNNING => {
//             SERVICE_RUNNING_FLAG.store(true, Ordering::SeqCst);
//             return winapi::shared::winerror::NOERROR as DWORD;
//         }
//         _ => {
//             return ERROR_CALL_NOT_IMPLEMENTED;
//         }
//     }
// }

// fn run_service(service_status_handle: SERVICE_STATUS_HANDLE) -> Result<(), u32> {
//     // Create a channel to be able to poll a stop event from the service worker loop.
//     let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();
//     shutdown_tx.send(10);
//     // For demo purposes, this service sends a UDP packet once a second.
//     let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
//     let sender_addr = SocketAddr::new(loopback_ip, SENDER_PORT);
//     let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
//     let msg = PING_MESSAGE.as_bytes();
//     let socket = UdpSocket::bind(sender_addr).unwrap();

//     loop {
//         // Send a UDP packet
//         let _ = socket.send_to(msg, receiver_addr);

//         // Poll shutdown event.
//         match shutdown_rx.recv_timeout(std::time::Duration::from_secs(80)) {
//             Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
//             Err(mpsc::RecvTimeoutError::Timeout) => (),
//         };
//     }

//     Ok(())
// }

// use std::process::Command;
// use std::env;

// const SERVICE_NAME: &str = "Rusel_Cache_Service10";
// const SERVICE_DISPLAY_NAME: &str = "Russel Cache Service";
// const EXECUTABLE_NAME: &str = "russel.exe"; // Change this to the name of your compiled executable


// pub fn install_service_with_nssm() -> Result<(), String> {
//     let current_exe = env::current_exe().map_err(|e| format!("Failed to get current exe: {}", e))?;
//     let current_exe_str = current_exe.to_str().ok_or("Failed to convert path to string")?;
//     println!("{:?}",current_exe_str);
//     // Adjust the NSSM path if it's not in your PATH
//     let nssm_path = "C:\\Users\\DIAKO\\Desktop\\nssm-2.24\\win32";
    
//     // Install the service
//     let install_status = Command::new(nssm_path)
//         .arg("install")
//         .arg(SERVICE_NAME)
//         .arg(current_exe_str)
//         .status()
//         .map_err(|e| format!("Failed to execute NSSM install: {}", e))?;
    
//     if !install_status.success() {
//         return Err(format!("NSSM install failed with status: {}", install_status));
//     }

//     // Set the display name
//     let set_display_name_status = Command::new(nssm_path)
//         .arg("set")
//         .arg(SERVICE_NAME)
//         .arg("DisplayName")
//         .arg(SERVICE_DISPLAY_NAME)
//         .status()
//         .map_err(|e| format!("Failed to execute NSSM set DisplayName: {}", e))?;
    
//     if !set_display_name_status.success() {
//         return Err(format!("NSSM set DisplayName failed with status: {}", set_display_name_status));
//     }

//     // Optionally set the service description
//     let description = "This service runs Russel cache and checks health of the application";
//     let set_description_status = Command::new(nssm_path)
//         .arg("set")
//         .arg(SERVICE_NAME)
//         .arg("Description")
//         .arg(description)
//         .status()
//         .map_err(|e| format!("Failed to execute NSSM set Description: {}", e))?;
    
//     if !set_description_status.success() {
//         return Err(format!("NSSM set Description failed with status: {}", set_description_status));
//     }

//     Ok(())
// }
extern crate winapi;

use std::env;
use std::ffi::CString;
use std::ptr::null_mut;
use std::process::Command;
use winapi::um::shellapi::ShellExecuteA;

const SERVICE_NAME: &str = "russel test"; 
const SERVICE_DISPLAY_NAME: &str = "Your Service Display Name"; 
const SW_SHOW: i32 = 5;

pub fn install_service_with_nssm() -> Result<(), String> {
    let current_exe = env::current_exe().map_err(|e| format!("Failed to get current exe: {:?}", e))?;
    let current_exe_str = current_exe.to_str().ok_or("Failed to convert path to string")?;
    println!("{:?}", current_exe_str);
    
    let nssm_path = std::env::current_exe().unwrap().join("nssm");

    let command = format!("& \".\\nssm.exe\" install {} \"{}\"", SERVICE_NAME,current_exe_str);

    let command_cstr = CString::new(command).map_err(|e| e.to_string())?;
    let result = unsafe {
        ShellExecuteA(
             null_mut(),
            CString::new("runas").unwrap().as_ptr(), // Use "runas" to request elevated privileges
            CString::new(&nssm_path.to_str().unwrap()).unwrap().as_ptr() as *const i8,
            command_cstr.as_ptr(),
            null_mut(),
            SW_SHOW,
        )
    };

    if result as isize <= 32 {
        let error_message = match result as isize {
            0 => "The operation was unsuccessful.",
            2 => "The system cannot find the file specified.",
            5 => "Access is denied.",
            30 => "The other program is not responding.",
            // Add other cases as needed
            _ => "Unknown error code.",
        };
        return Err(format!("Failed to install russel service,error code: 0x{:X}, {}", result as isize, error_message));
    }

    // Set the display name
    let set_display_name_command = format!("set {} DisplayName {:?}", SERVICE_NAME, SERVICE_DISPLAY_NAME);
    let set_display_name_cstr = CString::new(set_display_name_command).map_err(|e| e.to_string())?;
    let set_display_name_result = unsafe {
        ShellExecuteA(
            null_mut(),
            CString::new("runas").unwrap().as_ptr(),
            set_display_name_cstr.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOW,
        )
    };

    if result as isize <= 32 {
        let error_message = match result as isize {
            0 => "The operation was unsuccessful.",
            2 => "The system cannot find the file specified.",
            5 => "Access is denied.",
            30 => "The other program is not responding.",
            // Add other cases as needed
            _ => "Unknown error code.",
        };
        return Err(format!("Failed to set description russel,error code: 0x{:X}, {}", result as isize, error_message));
    }


    // Optionally set the service description
    let description = "This service runs Russel cache and checks health of the application";
    let set_description_command = format!("set {} Description \"{}\"", SERVICE_NAME, description);
    let set_description_cstr = CString::new(set_description_command).map_err(|e| e.to_string())?;
    let set_description_result = unsafe {
        ShellExecuteA(
            null_mut(),
            CString::new("runas").unwrap().as_ptr(),
            set_description_cstr.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOW,
        )
    };

    if result as isize <= 32 {
        let error_message = match result as isize {
            0 => "The operation was unsuccessful.",
            2 => "The system cannot find the file specified.",
            5 => "Access is denied.",
            30 => "The other program is not responding.",
            // Add other cases as needed
            _ => "Unknown error code.",
        };
        return Err(format!("Failed to set optional args russel,error code: 0x{:X}, {}", result as isize, error_message));
    }


    Ok(())
}

