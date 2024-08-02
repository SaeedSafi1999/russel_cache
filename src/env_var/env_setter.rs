use std::io::Result;
use winreg::{enums::*, RegKey};

// pub fn set_user_environment_variable(key: &str, value: &str) -> Result<()> {
//     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//     let (env, _) = hkcu.create_subkey("Environment").unwrap();
//     env.set_value(&key, &value).unwrap();
//     Ok(())
// }
// pub fn set_user_path_environment_variable(path: &str) -> bool {
//     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//     let (env, _) = hkcu.create_subkey("Environment").unwrap();
//     let key = "Path";
//     let mut path_value: String = env.get_value(key).map_or(String::from(""), |v| v);
//     let path_exist = path_value.split(";").any(|v| v == path);
//     if path_exist == false {
//         path_value.push_str(path);
//         env.set_value(key, &path_value).unwrap();
//         return true;
//     }
//     return false;
// }
pub fn set_user_path_environment_variable(path: &str) -> std::io::Result<bool> {
    // Open the HKEY_CURRENT_USER registry key
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    // Open (or create if not exists) the "Environment" subkey
    let (env, _) = hkcu.create_subkey("Environment")?;
    
    let key = "Path";
    
    // Attempt to read the current Path value
    let mut path_value: String = match env.get_value(key) {
        Ok(value) => value,
        Err(_) => String::new(),
    };
    
    // Check if the given path already exists in the Path variable
    let path_exist = path_value.split(';').any(|v| v == path);
    if !path_exist {
        // Append the new path to the existing Path value, with a semicolon separator
        if !path_value.is_empty() && !path_value.ends_with(';') {
            path_value.push(';');
        }
        path_value.push_str(path);
        
        // Update the Path value in the registry
        env.set_value(key, &path_value)?;
        
        // Inform the system about the environment variable change
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