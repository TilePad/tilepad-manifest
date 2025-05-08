use thiserror::Error;

pub mod icons;
pub mod plugin;
pub mod system;
pub mod validation;

/// Errors that can occur when parsing the manifest
#[derive(Debug, Error)]
pub enum ManifestError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Validation(#[from] garde::Report),
}
