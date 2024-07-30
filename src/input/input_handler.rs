
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

// #[cfg(windows)]
// mod ping_service {
//     use std::{
//         ffi::OsString,
//         net::{IpAddr, SocketAddr, UdpSocket},
//         sync::mpsc,
//         time::Duration,
//     };
//     use windows_service::{
//         define_windows_service,
//         service::{
//             ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
//             ServiceType
//         },
//         service_control_handler::{self, ServiceControlHandlerResult},
//         service_dispatcher, Result,
//     };

//     const SERVICE_NAME: &str = "Russel";
//     const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

//     const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
//     const RECEIVER_PORT: u16 = 1234;
//     const PING_MESSAGE: &str = "ping\n";

//     pub fn run() -> Result<()> {
//         // Register generated `ffi_service_main` with the system and start the service, blocking
//         // this thread until the service is stopped.
//         service_dispatcher::start(SERVICE_NAME, ffi_service_main)
//     }

//     // Generate the windows service boilerplate.
//     // The boilerplate contains the low-level service entry function (ffi_service_main) that parses
//     // incoming service arguments into Vec<OsString> and passes them to user defined service
//     // entry (my_service_main).
//     define_windows_service!(ffi_service_main, my_service_main);

//     // Service entry function which is called on background thread by the system with service
//     // parameters. There is no stdout or stderr at this point so make sure to configure the log
//     // output to file if needed.
//     pub fn my_service_main(_arguments: Vec<OsString>) {
//         if let Err(_e) = run_service() {
//             // Handle the error, by logging or something.
//         }
//     }

//     pub fn run_service() -> Result<()> {
//         // Create a channel to be able to poll a stop event from the service worker loop.
//         let (shutdown_tx, shutdown_rx) = mpsc::channel();

//         // Define system service event handler that will be receiving service events.
//         let event_handler = move |control_event| -> ServiceControlHandlerResult {
//             match control_event {
//                 // Notifies a service to report its current status information to the service
//                 // control manager. Always return NoError even if not implemented.
//                 ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

//                 // Handle stop
//                 ServiceControl::Stop => {
//                     shutdown_tx.send(()).unwrap();
//                     ServiceControlHandlerResult::NoError
//                 }

//                 // treat the UserEvent as a stop request
//                 ServiceControl::UserEvent(code) => {
//                     if code.to_raw() == 130 {
//                         shutdown_tx.send(()).unwrap();
//                     }
//                     ServiceControlHandlerResult::NoError
//                 }

//                 _ => ServiceControlHandlerResult::NotImplemented,
//             }
//         };

//         // Register system service event handler.
//         // The returned status handle should be used to report service status changes to the system.
//         let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

//         // Tell the system that service is running
//         status_handle.set_service_status(windows_service::service::ServiceStatus {
//             service_type: SERVICE_TYPE,
//             current_state: ServiceState::Running,
//             controls_accepted: ServiceControlAccept::STOP,
//             exit_code: ServiceExitCode::Win32(0),
//             checkpoint: 0,
//             wait_hint: Duration::default(),
//             process_id: None,
//         })?;

//         // For demo purposes this service sends a UDP packet once a second.
//         let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
//         let sender_addr = SocketAddr::new(loopback_ip, 0);
//         let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
//         let msg = PING_MESSAGE.as_bytes();
//         let socket = UdpSocket::bind(sender_addr).unwrap();

//         loop {
//             let _ = socket.send_to(msg, receiver_addr);

//             // Poll shutdown event.
//             match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
//                 // Break the loop either upon stop or channel disconnect
//                 Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

//                 // Continue work if no events were received within the timeout
//                 Err(mpsc::RecvTimeoutError::Timeout) => (),
//             };
//         }

//         // Tell the system that service has stopped.
//         status_handle.set_service_status(ServiceStatus {
//             service_type: SERVICE_TYPE,
//             current_state: ServiceState::Stopped,
//             controls_accepted: ServiceControlAccept::empty(),
//             exit_code: ServiceExitCode::Win32(0),
//             checkpoint: 0,
//             wait_hint: Duration::default(),
//             process_id: None,
//         })?;

//         Ok(())
//     }
// }

/// /////////////

// const SERVICE_NAME: &str = "Russel_Cache";
// const SERVICE_DISPLAY_NAME: &str = "Russel_Service";
// const SERVICE_DESCRIPTION: &str = "A service for managing Russel Cache";

// fn install_service() -> windows_service::Result<()> {
//     use windows_service::{
//         service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
//         service_manager::{ServiceManager, ServiceManagerAccess},
//     };

//     let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
//     let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

//     // This example installs the service defined in `examples/ping_service.rs`.
//     // In the real world code you would set the executable path to point to your own binary
//     // that implements windows service.
//     let service_binary_path = ::std::env::current_exe()
//         .unwrap();

//     let service_info = ServiceInfo {
//         name: OsString::from(SERVICE_NAME),
//         display_name: OsString::from(SERVICE_DISPLAY_NAME),
//         service_type: ServiceType::OWN_PROCESS,
//         start_type: ServiceStartType::AutoStart,
//         error_control: ServiceErrorControl::Normal,
//         executable_path: service_binary_path,
//         launch_arguments: vec![],
//         dependencies: vec![],
//         account_name: None, // run as System
//         account_password: None,
//     };
//     let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
//     service.set_description("this is service for russel cache")?;
//     Ok(())
// }

fn start_service() -> windows_service:: Result<()> {
    let run_result = run().unwrap();
    Ok(())
}
extern crate winapi;

use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use winapi::um::winsvc::{
    StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerExW, SetServiceStatus,
    SERVICE_STATUS, SERVICE_TABLE_ENTRYW, SERVICE_ACCEPT_STOP, SERVICE_RUNNING,
    SERVICE_STOPPED, SERVICE_START_PENDING, SERVICE_STOP_PENDING, SERVICE_CONTROL_STOP,
    SERVICE_CONTROL_INTERROGATE, SERVICE_STATUS_HANDLE, SERVICE_CONTROL_PAUSE, SERVICE_CONTROL_CONTINUE,
    SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_PARAMCHANGE,
    SERVICE_CONTROL_NETBINDADD, SERVICE_CONTROL_NETBINDREMOVE, SERVICE_CONTROL_NETBINDENABLE,
    SERVICE_CONTROL_NETBINDDISABLE
};
use winapi::shared::minwindef::{DWORD, LPVOID, TRUE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::winerror::ERROR_CALL_NOT_IMPLEMENTED;
use winapi::um::winnt::SERVICE_WIN32_OWN_PROCESS;


const SERVICE_NAME: &str = "333";
const SERVICE_DISPLAY_NAME:&str ="2222";
const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
const SENDER_PORT :u16 = 2020;
const RECEIVER_PORT: u16 = 8080;
const PING_MESSAGE: &str = "ping";
static SERVICE_RUNNING_FLAG: AtomicBool = AtomicBool::new(false);

fn install_service() -> windows_service::Result<()> {
        use windows_service::{
            service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
            service_manager::{ServiceManager, ServiceManagerAccess},
        };
    
        let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    
        // This example installs the service defined in `examples/ping_service.rs`.
        // In the real world code you would set the executable path to point to your own binary
        // that implements windows service.
        let service_binary_path = ::std::env::current_exe().unwrap();
        println!("{:?}",service_binary_path);
    
        let service_info = ServiceInfo {
            name: OsString::from(SERVICE_NAME),
            display_name: OsString::from(SERVICE_DISPLAY_NAME),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::AutoStart,
            error_control: ServiceErrorControl::Normal,
            executable_path: service_binary_path,
            launch_arguments: vec![],
            dependencies: vec![],
            account_name: None, // run as System
            account_password: None,
        };
        let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
        service.set_description("this is service for russel cache")?;
        Ok(())
    }
    

pub fn run() -> Result<(), u32> {
    // Convert the service name to a wide string
    let service_name: Vec<u16> = OsString::from(SERVICE_NAME).encode_wide().chain(Some(0)).collect();
    let service_table: [SERVICE_TABLE_ENTRYW; 2] = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: service_name.as_ptr(),
            lpServiceProc: Some(service_main),
        },
        SERVICE_TABLE_ENTRYW {
            lpServiceName: null_mut(),
            lpServiceProc: None,
        },
    ];

    // Start the service control dispatcher
    let success = unsafe { StartServiceCtrlDispatcherW(service_table.as_ptr()) };
    if success == TRUE {
        Ok(())
    } else {
        Err(unsafe { GetLastError() })
    }
}

// Service main function
unsafe extern "system" fn service_main(_argc: DWORD, _argv: *mut *mut u16) {
    // Register the service control handler
    let service_name: Vec<u16> = OsString::from(SERVICE_NAME).encode_wide().chain(Some(0)).collect();
    let service_status_handle = RegisterServiceCtrlHandlerExW(
        service_name.as_ptr(),
        Some(service_control_handler),
        null_mut(),
    );

    if service_status_handle.is_null() {
        return;
    }

    // Set the service status to start pending
    let mut service_status = SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: SERVICE_START_PENDING,
        dwControlsAccepted: 0,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 30000,
    };
    SetServiceStatus(service_status_handle, &mut service_status);

    // Set the service status to running
    service_status.dwCurrentState = SERVICE_RUNNING;
    service_status.dwControlsAccepted = SERVICE_ACCEPT_STOP;
    service_status.dwWaitHint = 0;
    SetServiceStatus(service_status_handle, &mut service_status);

    SERVICE_RUNNING_FLAG.store(true, Ordering::SeqCst);

    // Run the service
    if let Err(e) = run_service(service_status_handle) {
        // Handle the error, by logging or something.
    }

    // Set the service status to stopped
    service_status.dwCurrentState = SERVICE_STOPPED;
    service_status.dwControlsAccepted = 0;
    SetServiceStatus(service_status_handle, &mut service_status);
}

// Service control handler
unsafe extern "system" fn service_control_handler(
    control: DWORD,
    _event_type: DWORD,
    _event_data: LPVOID,
    _context: LPVOID,
) -> DWORD {
    match control {
        SERVICE_CONTROL_STOP => {
            // Set the service flag to indicate it's stopping
            SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);

            // Inform the service control manager that the service is stopping
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_PAUSE => {
            // Handle the pause request (implement logic if needed)
            // For example, you could stop processing or pause work
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_CONTINUE => {
            // Handle the continue request (implement logic if needed)
            // For example, you could resume processing or continue work
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_INTERROGATE => {
            // Handle the interrogation request (provide status if needed)
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_SHUTDOWN => {
            // Handle shutdown request (same as stop)
            SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_PARAMCHANGE => {
            // Handle parameter change request (implement logic if needed)
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_NETBINDADD |
        SERVICE_CONTROL_NETBINDREMOVE |
        SERVICE_CONTROL_NETBINDENABLE |
        SERVICE_CONTROL_NETBINDDISABLE => {
            // Handle network binding changes if applicable
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        _ => {
            // For unknown control codes, return the error code
            return ERROR_CALL_NOT_IMPLEMENTED;
        }
    }
}

pub fn run_service(service_status_handle: SERVICE_STATUS_HANDLE) -> Result<(), u32> {
    // Create a channel to be able to poll a stop event from the service worker loop.
    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    // For demo purposes this service sends a UDP packet once a second.
    let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
    let sender_addr = SocketAddr::new(loopback_ip, SENDER_PORT);
    let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
    let msg = PING_MESSAGE.as_bytes();
    let socket = UdpSocket::bind(sender_addr).unwrap();

    loop {
        shutdown_tx.send(1);
        let _ = socket.send_to(msg, receiver_addr);
        
        // Poll shutdown event.
        match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
            // Break the loop either upon stop or channel disconnect
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

            // Continue work if no events were received within the timeout
            Err(mpsc::RecvTimeoutError::Timeout) => (),
        };
    }

    Ok(())
}

fn stop_service() -> windows_service::Result<()> {
    let manager_access = windows_service::service_manager::ServiceManagerAccess::CONNECT;
    let service_manager = windows_service::service_manager::ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = windows_service::service::ServiceAccess::STOP;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    service.stop()?;
    Ok(())
}

fn delete_service() -> windows_service::Result<()> {
    let manager_access = windows_service::service_manager::ServiceManagerAccess::CONNECT;
    let service_manager = windows_service::service_manager::ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = windows_service::service::ServiceAccess::DELETE;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    service.delete()?;
    Ok(())
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
                    Ok(_) => println!("Service deleted successfully. restart your system for apply cahnge"),
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