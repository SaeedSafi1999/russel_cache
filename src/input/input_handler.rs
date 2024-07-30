
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

#[cfg(windows)]
mod ping_service {
    use std::{
        ffi::OsString,
        net::{IpAddr, SocketAddr, UdpSocket},
        sync::mpsc,
        time::Duration,
    };
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher, Result,
    };

    const SERVICE_NAME: &str = "Russel";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

    const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
    const RECEIVER_PORT: u16 = 1234;
    const PING_MESSAGE: &str = "ping\n";

    pub fn run() -> Result<()> {
        // Register generated `ffi_service_main` with the system and start the service, blocking
        // this thread until the service is stopped.
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    // Generate the windows service boilerplate.
    // The boilerplate contains the low-level service entry function (ffi_service_main) that parses
    // incoming service arguments into Vec<OsString> and passes them to user defined service
    // entry (my_service_main).
    define_windows_service!(ffi_service_main, my_service_main);

    // Service entry function which is called on background thread by the system with service
    // parameters. There is no stdout or stderr at this point so make sure to configure the log
    // output to file if needed.
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // Handle the error, by logging or something.
        }
    }

    pub fn run_service() -> Result<()> {
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Define system service event handler that will be receiving service events.
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }

                // treat the UserEvent as a stop request
                ServiceControl::UserEvent(code) => {
                    if code.to_raw() == 130 {
                        shutdown_tx.send(()).unwrap();
                    }
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        // Register system service event handler.
        // The returned status handle should be used to report service status changes to the system.
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

        // Tell the system that service is running
        status_handle.set_service_status(windows_service::service::ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // For demo purposes this service sends a UDP packet once a second.
        let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
        let sender_addr = SocketAddr::new(loopback_ip, 0);
        let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
        let msg = PING_MESSAGE.as_bytes();
        let socket = UdpSocket::bind(sender_addr).unwrap();

        loop {
            let _ = socket.send_to(msg, receiver_addr);

            // Poll shutdown event.
            match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                // Break the loop either upon stop or channel disconnect
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                // Continue work if no events were received within the timeout
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            };
        }

        // Tell the system that service has stopped.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        Ok(())
    }
}

/// /////////////

const SERVICE_NAME: &str = "Russel_Cache";
const SERVICE_DISPLAY_NAME: &str = "Russel_Service";
const SERVICE_DESCRIPTION: &str = "A service for managing Russel Cache";

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
    let service_binary_path = ::std::env::current_exe()
        .unwrap()
        .with_file_name("russel.exe");

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
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

fn start_service() -> windows_service:: Result<()> {
    ping_service::run();
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