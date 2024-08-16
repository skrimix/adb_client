use std::{fmt::Display, str::FromStr};

use crate::RustADBError;

/// Represents the connection state of the device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceState {
    /// The device is not connected to adb or is not responding.
    Offline,
    /// The device is now connected to the adb server. Note that this state does not imply that the Android system is fully booted and operational because the device connects to adb while the system is still booting. However, after boot-up, this is the normal operational state of an device.
    Device,
    /// The device is in recovery mode.
    Recovery,
    /// Insufficient permissions to communicate with the device.
    NoPermissions,
    /// The device is in sideload mode.
    Sideload,
    /// There is no device connected.
    NoDevice,
    /// Device is being authorized.
    Authorizing,
    /// The device is unauthorized.
    Unauthorized,
}

impl Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceState::Offline => write!(f, "offline"),
            DeviceState::Device => write!(f, "device"),
            DeviceState::Recovery => write!(f, "recovery"),
            DeviceState::NoPermissions => write!(f, "no permissions"),
            DeviceState::Sideload => write!(f, "sideload"),
            DeviceState::NoDevice => write!(f, "no device"),
            DeviceState::Authorizing => write!(f, "authorizing"),
            DeviceState::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

impl FromStr for DeviceState {
    type Err = RustADBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercased = s.to_ascii_lowercase();
        match lowercased.as_str() {
            "offline" => Ok(Self::Offline),
            "device" => Ok(Self::Device),
            "recovery" => Ok(Self::Recovery),
            "no permissions" => Ok(Self::NoPermissions),
            "sideload" => Ok(Self::Sideload),
            "no device" => Ok(Self::NoDevice),
            "authorizing" => Ok(Self::Authorizing),
            "unauthorized" => Ok(Self::Unauthorized),
            _ => Err(RustADBError::UnknownDeviceState(lowercased)),
        }
    }
}
