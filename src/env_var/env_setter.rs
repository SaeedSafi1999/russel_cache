use std::io::Result;
use winreg::{enums::*, RegKey};

pub fn set_user_environment_variable(key: &str, value: &str) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); 
    env.set_value(&key, &value).unwrap();
    Ok(())
}
