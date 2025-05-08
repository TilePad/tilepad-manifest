//! # Icons
//!
//! Manifest definition for icon packs

use crate::{ManifestError, validation::validate_id};
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Manifest for an icon pack
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct IconsManifest {
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
    #[inline]
    pub fn parse(value: &str) -> Result<IconsManifest, ManifestError> {
        Self::try_from(value)
    }
}

/// Icon within an icon collection
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Icon {
    /// Path to the icon file
    #[garde(length(min = 1))]
    pub path: String,

    /// Name of the icon
    #[garde(length(min = 1))]
    pub name: String,
}

/// Icon pack details for the pack
#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct MIconPack {
    /// Unique ID of the icon pack (e.g com.jacobtread.tilepad.obs)
    #[garde(dive)]
    pub id: IconPackId,
    /// Name of the icon pack
    #[garde(length(min = 1))]
    pub name: String,
    /// Version of the icon pack
    #[garde(length(min = 1))]
    pub version: String,
    /// List of authors for the pack
    #[garde(skip)]
    pub authors: Vec<String>,
    /// Description of the pack
    #[garde(skip)]
    pub description: Option<String>,
    /// Icon for the pack
    #[garde(skip)]
    pub icon: Option<String>,
}

/// Unique ID for an icon pack
///
/// Uses reverse domain syntax (i.e com.example.my-pack)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[garde(transparent)]
#[serde(transparent)]
pub struct IconPackId(#[garde(custom(validate_id))] pub String);

impl IconPackId {
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
