use std::fmt::Display;

use super::RebootType;
use std::net::SocketAddrV4;

pub(crate) enum AdbServerCommand {
    // Host commands
    Version,
    Kill,
    Devices,
    DevicesLong,
    TrackDevices,
    HostFeatures,
    Connect(SocketAddrV4),
    Disconnect(SocketAddrV4),
    Pair(SocketAddrV4, u32),
    TransportAny,
    TransportSerial(String),
    // Local commands
    ShellCommand(String),
    Shell,
    FrameBuffer,
    Sync,
    Reboot(RebootType),
    TcpIp(u16),
}

impl Display for AdbServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdbServerCommand::Version => write!(f, "host:version"),
            AdbServerCommand::Kill => write!(f, "host:kill"),
            AdbServerCommand::Devices => write!(f, "host:devices"),
            AdbServerCommand::DevicesLong => write!(f, "host:devices-l"),
            AdbServerCommand::Sync => write!(f, "sync:"),
            AdbServerCommand::TrackDevices => write!(f, "host:track-devices"),
            AdbServerCommand::TransportAny => write!(f, "host:transport-any"),
            AdbServerCommand::TransportSerial(serial) => write!(f, "host:transport:{serial}"),
            AdbServerCommand::ShellCommand(command) => match std::env::var("TERM") {
                Ok(term) => write!(f, "shell,TERM={term},raw:{command}"),
                Err(_) => write!(f, "shell,raw:{command}"),
            },
            AdbServerCommand::Shell => match std::env::var("TERM") {
                Ok(term) => write!(f, "shell,TERM={term},raw:"),
                Err(_) => write!(f, "shell,raw:"),
            },
            AdbServerCommand::HostFeatures => write!(f, "host:features"),
            AdbServerCommand::Reboot(reboot_type) => {
                write!(f, "reboot:{reboot_type}")
            }
            AdbServerCommand::Connect(addr) => write!(f, "host:connect:{}", addr),
            AdbServerCommand::Disconnect(addr) => write!(f, "host:disconnect:{}", addr),
            AdbServerCommand::Pair(addr, code) => {
                write!(f, "host:pair:{code}:{}", addr)
            }
            AdbServerCommand::FrameBuffer => write!(f, "framebuffer:"),
            AdbServerCommand::TcpIp(port) => write!(f, "tcpip:{port}"),
        }
    }
}
