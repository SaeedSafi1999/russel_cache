
use std::process::Command;
use std::env;
const SERVICE_NAME: &str = "MyServiceName";

pub fn install_service_with_nssm() -> Result<(), String> {
    let current_exe = env::current_exe().map_err(|e| format!("Failed to get current exe: {:?}", e))?;
    let current_exe_str = current_exe.to_str().ok_or("Failed to convert path to string")?;

    let nssm_path = current_exe.parent().unwrap().join("nssm");
    println!("{:?}",nssm_path);
    if !nssm_path.exists() {
        return Err(format!("nssm.exe not found at path: {:?}", nssm_path));
    }

    let nssm_path_str = nssm_path.to_str().ok_or("Failed to convert nssm path to string")?;

    let install_status = Command::new("powershell.exe")
        .arg("-Command")
        .arg("Start-Process")
        .arg(nssm_path_str)
        .arg("-ArgumentList")
        .arg(format!("nssm.exe install {}, \"{}\"", SERVICE_NAME, current_exe_str))
        .arg("-Verb")
        .arg("RunAs")  // This requests elevation
        .status()
        .map_err(|e| format!("Failed to execute nssm install command as admin: {:?}", e))?;

    if !install_status.success() {
        return Err(format!("nssm install command failed with exit code: {}", install_status.code().unwrap_or(-1)));
    }

    Ok(())
}
