use winreg::{enums::*, RegKey};

pub fn set_user_path_environment_variable(path: &str) -> std::io::Result<bool> {
    // Open the HKEY_CURRENT_USER registry key
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let (env, _) = hkcu.create_subkey("Environment")?;
    
    let key = "Path";
    
    let mut path_value: String = match env.get_value(key) {
        Ok(value) => value,
        Err(_) => String::new(),
    };
    
    let path_exist = path_value.split(';').any(|v| v == path);
    if !path_exist {
        if !path_value.is_empty() && !path_value.ends_with(';') {
            path_value.push(';');
        }
        path_value.push_str(path);
        
        env.set_value(key, &path_value)?;
        
        inform_system_about_environment_variable_change()?;
        
        return Ok(true);
    }
    
    Ok(false)
}

fn inform_system_about_environment_variable_change() -> std::io::Result<()> {
    use std::ptr;
    use winapi::um::winuser::{SendMessageTimeoutA, SMTO_ABORTIFHUNG, HWND_BROADCAST, WM_SETTINGCHANGE};

    // Notify all windows about the environment variable change
    let result = unsafe {
        SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            "Environment\0".as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            5000,
            ptr::null_mut(),
        )
    };

    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}