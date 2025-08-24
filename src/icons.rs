//! # Icons
//!
//! Manifest definition for icon packs

use crate::{ManifestError, validation::validate_id};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

/// Manifest for an icon pack
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct IconsManifest {
    /// Definition for the icon pack details
    #[garde(dive)]
    pub icons: MIconPack,
}

impl TryFrom<&str> for IconsManifest {
    type Error = ManifestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let manifest: IconsManifest = serde_json::from_str(value)?;
        manifest.validate()?;
        Ok(manifest)
    }
}

impl TryFrom<&[u8]> for IconsManifest {
    type Error = ManifestError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let manifest: IconsManifest = serde_json::from_slice(value)?;
        manifest.validate()?;
        Ok(manifest)
    }
}

impl IconsManifest {
    /// Parse an [IconsManifest] from a string
    #[inline]
    pub fn parse(value: &str) -> Result<IconsManifest, ManifestError> {
        Self::try_from(value)
    }
}

/// Icon within an icon collection
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct Icon {
    /// Path to the icon file
    #[garde(length(min = 1))]
    #[schemars(example = "images/icon.svg")]
    pub path: String,

    /// Name of the icon
    #[garde(length(min = 1))]
    #[schemars(example = "My Icon")]
    pub name: String,
}

/// Icon pack details for the pack
#[derive(Debug, Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct MIconPack {
    /// Unique ID of the icon pack (e.g com.jacobtread.tilepad.obs)
    #[garde(dive)]
    pub id: IconPackId,
    /// Name of the icon pack
    #[garde(length(min = 1))]
    #[schemars(example = "My Icon Pack")]
    pub name: String,
    /// Version of the icon pack, semver compatible version number
    #[garde(length(min = 1))]
    #[schemars(example = "0.1.0")]
    pub version: String,
    /// List of authors for the pack
    #[garde(skip)]
    #[schemars(example = ["Example Author 1", "Example Author 2"])]
    pub authors: Vec<String>,
    /// Description of the pack
    #[garde(skip)]
    #[schemars(example = "My plugin that performs my actions")]
    pub description: Option<String>,
    /// Icon for the pack
    #[garde(skip)]
    #[schemars(example = "images/icon.svg")]
    pub icon: Option<String>,
}

/// Unique ID for an icon pack
///
/// Uses reverse domain syntax (i.e com.example.my-pack)
#[derive(
    Debug, Clone, Serialize, Deserialize, Validate, Hash, PartialEq, Eq, PartialOrd, Ord, JsonSchema,
)]
#[garde(transparent)]
#[serde(transparent)]
#[schemars(example = &"com.example.my-pack")]
pub struct IconPackId(#[garde(custom(validate_id))] pub String);

impl IconPackId {
    /// Get the inner icon pack ID as a [str] slice
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for IconPackId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for IconPackId {
    type Error = garde::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl FromStr for IconPackId {
    type Err = garde::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = IconPackId(s.to_string());
        value.validate()?;
        Ok(value)
    }
}

impl Display for IconPackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
