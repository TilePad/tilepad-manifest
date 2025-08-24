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

#[test]
fn generate_schema() {
    use schemars::generate::SchemaSettings;
    use std::path::Path;

    let schemas_path = Path::new("schemas");
    std::fs::create_dir_all(schemas_path).unwrap();

    let schema_path = schemas_path.join("icons.json");
    let generator = SchemaSettings::draft07().into_generator();
    let schema = generator.into_root_schema_for::<icons::IconsManifest>();
    std::fs::write(schema_path, serde_json::to_string_pretty(&schema).unwrap()).unwrap();

    let schema_path = schemas_path.join("plugins.json");
    let generator = SchemaSettings::draft07().into_generator();
    let schema = generator.into_root_schema_for::<plugin::PluginManifest>();
    std::fs::write(schema_path, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}
