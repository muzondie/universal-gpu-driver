use std::process::Command;
use crate::error::Error;

pub fn install_driver(path: &str) -> Result<(), Error> {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Expand-Archive -Path '{}' -DestinationPath C:\\ugd_temp", path),
        ])
        .output()?;

    if !output.status.success() {
        return Err(Error::InstallationFailed);
    }

    Ok(())
}