use std::io::Result;
use winreg::{enums::*, RegKey};

pub fn set_user_environment_variable(key: &str, value: &str) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();
    env.set_value(&key, &value).unwrap();
    Ok(())
}
pub fn set_user_path_environment_variable(path: &str) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap();
    let key = "Path";
    let mut path_value: String = env.get_value(key).map_or(String::from(""), |v| v);
    let path_exist = path_value.split(";").any(|v| v == path);
    if path_exist == false {
        println!("{:?}", path_value);
        path_value.push_str(path);
        println!("{:?}", path_value);
        env.set_value(key, &path_value).unwrap();

    }

    Ok(())
}
