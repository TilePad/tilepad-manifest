//! # Icons
//!
//! Manifest definition for icon packs

use std::str::FromStr;

use garde::Validate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IconsError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Validation(#[from] garde::Report),
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Icon {
    /// Path to the icon file
    #[garde(length(min = 1))]
    pub path: String,

    /// Name of the icon
    #[garde(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Manifest {
    #[garde(dive)]
    pub icons: IconsManifest,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct IconsManifest {
    /// Unique ID of the icon pack (e.g com.jacobtread.tilepad.obs)
    #[garde(dive)]
    pub id: IconPackId,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(length(min = 1))]
    pub version: String,
    #[garde(skip)]
    pub authors: Vec<String>,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(skip)]
    pub icon: Option<String>,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Validation(#[from] garde::Report),
}

impl Manifest {
    pub fn parse(value: &str) -> Result<Manifest, ManifestError> {
        let manifest: Manifest = serde_json::from_str(value)?;
        manifest.validate()?;
        Ok(manifest)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[garde(transparent)]
#[serde(transparent)]
pub struct IconPackId(#[garde(custom(is_valid_icon_pack_name))] pub String);

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

/// Separators allowed within names
static NAME_SEPARATORS: [char; 2] = ['-', '_'];

// Validates that a plugin name is valid
fn is_valid_icon_pack_name(value: &str, _context: &()) -> garde::Result {
    let parts = value.split('.');

    for part in parts {
        // Must start with a letter
        if !part.starts_with(|char: char| char.is_ascii_alphabetic()) {
            return Err(garde::Error::new(
                "segment must start with a ascii alphabetic character",
            ));
        }

        // Must only contain a-zA-Z0-9_-
        if !part
            .chars()
            .all(|char| char.is_alphanumeric() || NAME_SEPARATORS.contains(&char))
        {
            return Err(garde::Error::new(
                "icon pack name domain segment must only contain alpha numeric values and _ or -",
            ));
        }

        // Must not end with - or _
        if part.ends_with(NAME_SEPARATORS) {
            return Err(garde::Error::new(
                "icon pack name domain segment must not end with _ or -",
            ));
        }
    }

    Ok(())
}
