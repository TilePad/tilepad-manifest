//! # Plugin
//!
//! Manifest definition for plugins

use crate::{
    ManifestError,
    system::{Arch, OperatingSystem, platform_arch, platform_os},
    validation::{validate_color, validate_id, validate_name},
};
use garde::Validate;
use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{fmt::Display, str::FromStr};

/// Unique ID for a plugin
///
/// Uses reverse domain syntax (i.e com.example.my-plugin)
#[derive(
    Debug, Clone, Serialize, Deserialize, Validate, Hash, PartialEq, Eq, PartialOrd, Ord, JsonSchema,
)]
#[garde(transparent)]
#[serde(transparent)]
pub struct PluginId(#[garde(custom(validate_id))] pub String);

impl TryFrom<String> for PluginId {
    type Error = garde::Report;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl FromStr for PluginId {
    type Err = garde::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = PluginId(s.to_string());
        value.validate()?;
        Ok(value)
    }
}

impl Display for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PluginId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for PluginId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Version range of a node runtime, it's recommended that you leave
/// this as the default unless you explicitly need a specific version
/// and its features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(transparent)]
#[schemars(with = "String", example = "=22.18.0")]
pub struct BinaryNodeVersion(pub node_semver::Range);

impl AsRef<node_semver::Range> for BinaryNodeVersion {
    fn as_ref(&self) -> &node_semver::Range {
        &self.0
    }
}

/// Manifest file format for plugins
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct PluginManifest {
    /// Details about the plugin itself
    #[garde(dive)]
    pub plugin: MPlugin,

    /// Details for running the plugin
    /// (Option not specified for internal plugins)
    #[garde(dive)]
    pub bin: Option<MBin>,

    /// Category for the manifest actions
    #[garde(dive)]
    pub category: MCategory,

    /// Map of available plugin actions
    #[garde(dive)]
    pub actions: ActionMap,
}

impl TryFrom<&str> for PluginManifest {
    type Error = ManifestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let manifest: PluginManifest = serde_json::from_str(value)?;
        manifest.validate()?;
        Ok(manifest)
    }
}

impl TryFrom<&[u8]> for PluginManifest {
    type Error = ManifestError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let manifest: PluginManifest = serde_json::from_slice(value)?;
        manifest.validate()?;
        Ok(manifest)
    }
}

impl PluginManifest {
    /// Parse a plugin manifest from a string
    #[inline]
    pub fn parse(value: &str) -> Result<PluginManifest, ManifestError> {
        Self::try_from(value)
    }
}

/// Plugin details section of the manifest
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct MPlugin {
    /// Unique ID of the plugin (e.g com.jacobtread.tilepad.obs)
    #[garde(dive)]
    #[schemars(example = "com.jacobtread.tilepad.obs")]
    pub id: PluginId,
    /// Name of the plugin
    #[garde(length(min = 1))]
    #[schemars(example = "Example Plugin")]
    pub name: String,
    /// Current version of the plugin, semver compatible version number
    #[garde(length(min = 1))]
    #[schemars(example = "0.1.0")]
    pub version: String,
    /// List of authors for the plugin
    #[garde(inner(length(min = 1)))]
    #[schemars(example = ["Example Author 1", "Example Author 2"])]
    pub authors: Vec<String>,
    /// Description of what the plugin does
    #[garde(skip)]
    #[schemars(example = "My plugin that performs my actions")]
    pub description: Option<String>,
    /// Icon for the plugin
    #[garde(skip)]
    #[schemars(example = "images/icon.svg")]
    pub icon: Option<String>,
    /// Internal Field - Determines whether the plugin is "internal" to tilepad and
    /// bundled (Prevents uninstalling and some other features)
    #[garde(skip)]
    #[schemars(skip)]
    pub internal: Option<bool>,
}

/// Ordered map of actions defined within the plugin
///
/// Keys must be unique to each action following
/// the [a-zA-Z_-] format (i.e example_action, my-action, MyAction)
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ActionMap(pub IndexMap<ActionId, ManifestAction>);

impl AsRef<IndexMap<ActionId, ManifestAction>> for ActionMap {
    fn as_ref(&self) -> &IndexMap<ActionId, ManifestAction> {
        &self.0
    }
}

/// Definition of the category to place the plugin actions within
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct MCategory {
    /// Label for the category in the actions sidebar
    #[garde(length(min = 1))]
    #[schemars(example = "My Category")]
    pub label: String,
    /// Icon to show in the actions sidebar
    #[garde(skip)]
    #[schemars(example = "images/icon.svg")]
    pub icon: Option<String>,
}

/// Name of an action
///
/// Must be [a-zA-Z_-] (i.e example_action, my-action, MyAction)
#[derive(
    Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema,
)]
#[garde(transparent)]
#[serde(transparent)]
#[schemars(example = &"example_action")]
pub struct ActionId(#[garde(custom(validate_name))] pub String);

impl ActionId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for ActionId {
    type Error = garde::Report;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl FromStr for ActionId {
    type Err = garde::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = ActionId(s.to_string());
        value.validate()?;
        Ok(value)
    }
}

impl AsRef<str> for ActionId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ActionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Manifest action definition
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct ManifestAction {
    /// Label for the action, shown in the sidebar
    #[garde(length(min = 1))]
    #[schemars(example = "My Action")]
    pub label: String,

    /// Icon for the action, shown in the sidebar and
    /// used as the default icon when added to the grid
    #[garde(skip)]
    #[schemars(example = "images/icon.svg")]
    pub icon: Option<String>,

    /// Path to a "display" HTML file that can be used to make
    /// the tile into a display tile
    #[garde(skip)]
    #[schemars(example = "display/my-display.display.html")]
    pub display: Option<String>,

    /// Default options for the icon when added to the grid
    /// as a tile
    #[garde(dive)]
    pub icon_options: Option<ManifestActionIconOptions>,

    /// Description for the action, shown as a tooltip when hovering
    /// the action
    #[garde(skip)]
    #[schemars(example = "My action")]
    pub description: Option<String>,

    /// Path to the inspector HTML file to use for configuring the action
    #[garde(skip)]
    #[schemars(example = "inspector/index.html")]
    pub inspector: Option<String>,
}

/// Default options for an action icon
#[skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(default)]
pub struct ManifestActionIconOptions {
    /// Padding in pixels to pad the icon with
    #[garde(skip)]
    pub padding: Option<u32>,

    /// Color for the tile background behind the icon
    #[garde(inner(custom(validate_color)))]
    #[schemars(example = "#ffffff")]
    pub background_color: Option<String>,

    /// Color of the tile border
    #[garde(inner(custom(validate_color)))]
    #[schemars(example = "#ffffff")]
    pub border_color: Option<String>,
}

/// Type of binary the plugin program is using
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
#[serde(untagged)]
pub enum MBin {
    /// Program uses the node runtime
    Node {
        #[garde(dive)]
        node: MBinNode,
    },

    /// Program uses a native binary
    Native {
        #[garde(dive)]
        native: Vec<MBinNative>,
    },
}

/// Node "binary" which uses a node runtime to execute the js script
/// at the provided `entrypoint`
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct MBinNode {
    /// Entrypoint for the program
    ///
    /// e.g bin/index.js
    #[garde(length(min = 1))]
    #[schemars(example = "bin/index.js")]
    pub entrypoint: String,

    /// Version of node the program should run using.
    ///
    /// It's recommended that you leave
    /// this as the default unless you explicitly need a specific version
    /// and its features
    #[garde(skip)]
    #[serde(default = "default_node_version")]
    pub version: BinaryNodeVersion,
}

/// Default node version to use
fn default_node_version() -> BinaryNodeVersion {
    BinaryNodeVersion(
        node_semver::Range::parse("=22.18.0").expect("hardcoded range should be valid"),
    )
}

/// Native binary for a specific os + arch combo, contains a
/// path to the binary
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct MBinNative {
    // Target OS this binary should be used for
    #[garde(skip)]
    pub os: OperatingSystem,

    /// Target Arch this binary should be used for
    #[garde(skip)]
    pub arch: Arch,

    /// Path to the executable file
    #[garde(length(min = 1))]
    #[schemars(example = "bin/example.exe")]
    pub path: String,
}

impl MBinNative {
    // Check if the binary is usable on the provided OS and Arch combination
    pub fn is_usable(&self, os: &OperatingSystem, arch: &Arch) -> bool {
        self.os.eq(os) && self.arch.eq(arch)
    }

    // Find a binary thats usable on the provided OS and Arch combination
    pub fn find_usable<'a>(
        options: &'a [MBinNative],
        os: &OperatingSystem,
        arch: &Arch,
    ) -> Option<&'a Self> {
        options.iter().find(|bin| bin.is_usable(os, arch))
    }

    // Find a binary compatible with the current OS and Arch
    pub fn find_current(options: &[MBinNative]) -> Option<&Self> {
        let os = platform_os();
        let arch = platform_arch();
        Self::find_usable(options, &os, &arch)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_usable_true_when_matches() {
        let bin = MBinNative {
            os: OperatingSystem::Linux,
            arch: Arch::X64,
            path: "bin/linux-x64".to_string(),
        };
        assert!(bin.is_usable(&OperatingSystem::Linux, &Arch::X64));
    }

    #[test]
    fn test_is_usable_false_when_os_mismatch() {
        let bin = MBinNative {
            os: OperatingSystem::Linux,
            arch: Arch::X64,
            path: "bin/linux-x64".to_string(),
        };
        assert!(!bin.is_usable(&OperatingSystem::Windows, &Arch::X64));
    }

    #[test]
    fn test_is_usable_false_when_arch_mismatch() {
        let bin = MBinNative {
            os: OperatingSystem::Linux,
            arch: Arch::X64,
            path: "bin/linux-x64".to_string(),
        };
        assert!(!bin.is_usable(&OperatingSystem::Linux, &Arch::X86));
    }

    #[test]
    fn test_find_usable_finds_correct_bin() {
        let bins = vec![
            MBinNative {
                os: OperatingSystem::Windows,
                arch: Arch::X64,
                path: "bin/win-x64".to_string(),
            },
            MBinNative {
                os: OperatingSystem::Linux,
                arch: Arch::X64,
                path: "bin/linux-x64".to_string(),
            },
        ];
        let result = MBinNative::find_usable(&bins, &OperatingSystem::Linux, &Arch::X64);
        assert!(result.is_some());
        assert_eq!(result.unwrap().path, "bin/linux-x64");
    }

    #[test]
    fn test_find_usable_returns_none_if_no_match() {
        let bins = vec![
            MBinNative {
                os: OperatingSystem::Windows,
                arch: Arch::X64,
                path: "bin/win-x64".to_string(),
            },
            MBinNative {
                os: OperatingSystem::MacOs,
                arch: Arch::Arm64,
                path: "bin/macos-arm64".to_string(),
            },
        ];
        let result = MBinNative::find_usable(&bins, &OperatingSystem::Linux, &Arch::X64);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_usable_returns_first_match() {
        let bins = vec![
            MBinNative {
                os: OperatingSystem::Linux,
                arch: Arch::X64,
                path: "bin/linux-x64-v1".to_string(),
            },
            MBinNative {
                os: OperatingSystem::Linux,
                arch: Arch::X64,
                path: "bin/linux-x64-v2".to_string(),
            },
        ];
        let result = MBinNative::find_usable(&bins, &OperatingSystem::Linux, &Arch::X64);
        assert!(result.is_some());
        assert_eq!(result.unwrap().path, "bin/linux-x64-v1");
    }
}
