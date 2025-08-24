use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// Operating systems
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    #[strum(serialize = "windows")]
    Windows,
    #[strum(serialize = "macos")]
    MacOs,
    #[strum(serialize = "linux")]
    Linux,
}

impl Default for OperatingSystem {
    fn default() -> Self {
        platform_os()
    }
}

#[cfg(target_os = "windows")]
pub fn platform_os() -> OperatingSystem {
    OperatingSystem::Windows
}

#[cfg(target_os = "macos")]
pub fn platform_os() -> OperatingSystem {
    OperatingSystem::MacOs
}

#[cfg(target_os = "linux")]
pub fn platform_os() -> OperatingSystem {
    OperatingSystem::Linux
}

/// CPU architecture the binary is compiled as
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    #[strum(serialize = "x86")]
    X86,
    #[strum(serialize = "x64")]
    X64,
    #[strum(serialize = "arm")]
    Arm,
    #[strum(serialize = "arm64")]
    Arm64,
}

impl Default for Arch {
    fn default() -> Self {
        platform_arch()
    }
}

#[cfg(all(
    target_pointer_width = "64",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> Arch {
    Arch::X64
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> Arch {
    Arch::X86
}

#[cfg(all(
    target_pointer_width = "64",
    any(target_arch = "arm", target_arch = "aarch64")
))]
pub fn platform_arch() -> Arch {
    Arch::Arm64
}

#[cfg(all(
    target_pointer_width = "32",
    any(target_arch = "arm", target_arch = "aarch64")
))]
pub fn platform_arch() -> Arch {
    Arch::Arm
}
