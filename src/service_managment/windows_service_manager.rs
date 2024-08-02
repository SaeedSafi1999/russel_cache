extern crate winapi;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::ffi::OsString;
use std::sync::atomic::{AtomicBool, Ordering};
use winapi::um::winsvc::{
    StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerExW, SetServiceStatus,
    SERVICE_STATUS, SERVICE_TABLE_ENTRYW, SERVICE_ACCEPT_STOP, SERVICE_RUNNING,
    SERVICE_STOPPED, SERVICE_START_PENDING, SERVICE_CONTROL_STOP,
    SERVICE_CONTROL_INTERROGATE, SERVICE_STATUS_HANDLE, SERVICE_CONTROL_PAUSE, SERVICE_CONTROL_CONTINUE,
    SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_PARAMCHANGE,
    SERVICE_CONTROL_NETBINDADD, SERVICE_CONTROL_NETBINDREMOVE, SERVICE_CONTROL_NETBINDENABLE,
    SERVICE_CONTROL_NETBINDDISABLE
};
use winapi::shared::minwindef::{DWORD, LPVOID, TRUE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::winerror::ERROR_CALL_NOT_IMPLEMENTED;
use winapi::um::winnt::SERVICE_WIN32_OWN_PROCESS;


const SERVICE_NAME: &str = "888";
const SERVICE_DISPLAY_NAME:&str ="this service for running russel cache application";
const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
const SENDER_PORT :u16 = 1234;
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
    service.set_description("this service runs russel cache and check health of application")?;
    Ok(())
}


pub fn start_service() -> windows_service:: Result<()> {
    let run_result = run().unwrap();
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
            SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_PAUSE => {
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_CONTINUE => {
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_INTERROGATE => {
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_SHUTDOWN => {
            SERVICE_RUNNING_FLAG.store(false, Ordering::SeqCst);
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_PARAMCHANGE => {
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        SERVICE_CONTROL_NETBINDADD |
        SERVICE_CONTROL_NETBINDREMOVE |
        SERVICE_CONTROL_NETBINDENABLE |
        SERVICE_CONTROL_NETBINDDISABLE => {
            return winapi::shared::winerror::NOERROR.try_into().unwrap();
        }
        _ => {
            return ERROR_CALL_NOT_IMPLEMENTED;
        }
    }
}

fn run_service(service_status_handle: SERVICE_STATUS_HANDLE) -> Result<(), u32> {
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
        match shutdown_rx.recv_timeout(std::time::Duration::from_secs(1)) {
            // Break the loop either upon stop or channel disconnect
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

            // Continue work if no events were received within the timeout
            Err(mpsc::RecvTimeoutError::Timeout) => (),
        };
    }

    Ok(())
}
