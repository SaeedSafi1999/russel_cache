extern crate winapi;

use std::ffi::OsString;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use winapi::um::winsvc::{
    RegisterServiceCtrlHandlerExW, SetServiceStatus, StartServiceCtrlDispatcherW, SERVICE_ACCEPT_STOP, SERVICE_ACTIVE, SERVICE_CONTROL_INTERROGATE, SERVICE_CONTROL_PAUSE, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STATUS, SERVICE_STATUS_HANDLE, SERVICE_STOPPED, SERVICE_TABLE_ENTRYW
};
use winapi::shared::minwindef::{DWORD, LPVOID, TRUE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::winerror::ERROR_CALL_NOT_IMPLEMENTED;
const SERVICE_WIN32_OWN_PROCESS: DWORD = 0x00000010;
const SERVICE_NAME: &str = "Rusel_Cache_Service6";
const SERVICE_DISPLAY_NAME: &str = "russel cache services";
const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
const SENDER_PORT: u16 = 1234;
const RECEIVER_PORT: u16 = 8080;
const PING_MESSAGE: &str = "ping";
static SERVICE_RUNNING_FLAG: AtomicBool = AtomicBool::new(false);

pub fn install_service() -> windows_service::Result<()> {
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
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
        account_name: None,
        account_password: None,
    };

    let service = service_manager.create_service(&service_info, ServiceAccess::START | ServiceAccess::STOP | ServiceAccess::CHANGE_CONFIG | ServiceAccess::PAUSE_CONTINUE)?;
    service.set_description("This service runs Russel cache and checks health of the application")?;
    
    Ok(())
}

pub fn start_service() -> windows_service::Result<()> {
    run();
    Ok(())
}

pub fn stop_service() -> windows_service::Result<()> {
    let manager_access = windows_service::service_manager::ServiceManagerAccess::CONNECT;
    let service_manager = windows_service::service_manager::ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = windows_service::service::ServiceAccess::STOP;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    service.stop()?;
    Ok(())
}

pub fn delete_service() -> windows_service::Result<()> {
    let manager_access = windows_service::service_manager::ServiceManagerAccess::CONNECT;
    let service_manager = windows_service::service_manager::ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_access = windows_service::service::ServiceAccess::DELETE;
    let service = service_manager.open_service(SERVICE_NAME, service_access)?;
    service.delete()?;
    Ok(())
}

fn run() -> Result<(), u32> {
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
        eprintln!("Service encountered an error: {}", e);
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
            SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR as DWORD;
        }
        SERVICE_ACTIVE => {
            SERVICE_RUNNING_FLAG.store(true, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR as DWORD;
        }
        SERVICE_CONTROL_STOP => {
            return winapi::shared::winerror::NOERROR as DWORD;
            
        }
        SERVICE_CONTROL_PAUSE => {
            return winapi::shared::winerror::NOERROR as DWORD;
        }
        SERVICE_CONTROL_INTERROGATE => {
            return winapi::shared::winerror::NOERROR as DWORD;
        }
        SERVICE_CONTROL_SHUTDOWN => {
            SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR as DWORD;
        }
        SERVICE_RUNNING => {
            SERVICE_RUNNING_FLAG.store(true, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR as DWORD;
        }
        _ => {
            return ERROR_CALL_NOT_IMPLEMENTED;
        }
    }
}

fn run_service(service_status_handle: SERVICE_STATUS_HANDLE) -> Result<(), u32> {
    // Create a channel to be able to poll a stop event from the service worker loop.
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();
    shutdown_tx.send(10);
    // For demo purposes, this service sends a UDP packet once a second.
    let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
    let sender_addr = SocketAddr::new(loopback_ip, SENDER_PORT);
    let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
    let msg = PING_MESSAGE.as_bytes();
    let socket = UdpSocket::bind(sender_addr).unwrap();

    loop {
        // Send a UDP packet
        let _ = socket.send_to(msg, receiver_addr);

        // Poll shutdown event.
        match shutdown_rx.recv_timeout(std::time::Duration::from_secs(80)) {
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => (),
        };
    }

    Ok(())
}
