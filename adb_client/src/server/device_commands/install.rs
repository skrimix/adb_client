use crate::{models::InstallResult, ADBServerDevice, Result, RustADBError};
use std::fs::File;
use std::path::Path;

impl ADBServerDevice {
    /// Installs a package on the device (legacy push install)
    pub fn install(
        &mut self,
        apk_path: &str,
        reinstall: bool,
        grant_runtime_permissions: bool,
    ) -> Result<()> {
        let path = Path::new(apk_path);
        let mut input = File::open(path)?;
        let base_name = path.file_name().unwrap().to_str().unwrap();
        let remote_path = format!("/data/local/tmp/{base_name}");

        self.send(&mut input, &remote_path)?;

        let remote_path = format!("\"{remote_path}\"");
        let mut command = vec!["pm", "install"];
        if reinstall {
            command.push("-r");
        }
        if grant_runtime_permissions {
            command.push("-g");
        }
        command.push(&remote_path);

        let output = String::from_utf8(self.shell_command(command)?)?
            .trim()
            .to_string();

        let _ = self.shell_command(vec!["rm", &remote_path])?;

        let install_result = output.into();
        match install_result {
            InstallResult::Success => Ok(()),
            InstallResult::Error(message) => Err(RustADBError::PackageManagerError(message)),
        }
    }
}
