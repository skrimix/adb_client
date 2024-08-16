use crate::{models::AdbServerCommand, ADBServerDevice, Result};

impl ADBServerDevice {
    /// Restarts the device's adbd listening on TCP on the specified port.
    /// This will restart the USB connection too.
    pub fn tcpip(mut self, port: u16) -> Result<()> {
        let serial = self.identifier.clone();
        self.connect()?
            .send_adb_request(AdbServerCommand::TransportSerial(serial))?;

        self.get_transport_mut()
            .send_adb_request(AdbServerCommand::TcpIp(port))?;

        Ok(())
    }
}
