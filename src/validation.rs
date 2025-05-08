use crate::plugin::{ActionId, ActionMap};
use garde::{
    Path, Report, Validate,
    error::{Kind, PathComponentKind},
};

static NAME_SEPARATORS: [char; 2] = ['-', '_'];

/// Validate an ID (plugin ID or icon pack ID)
pub fn validate_id(value: &str, _context: &()) -> garde::Result {
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
                "name domain segment must only contain alpha numeric values and _ or -",
            ));
        }

        // Must not end with - or _
        if part.ends_with(NAME_SEPARATORS) {
            return Err(garde::Error::new(
                "name domain segment must not end with _ or -",
            ));
        }
    }

    Ok(())
}

// Validates that a action name is valid
pub fn validate_name(value: &str, _context: &()) -> garde::Result {
    // Must start with a letter
    if !value.starts_with(|char: char| char.is_ascii_alphabetic()) {
        return Err(garde::Error::new(
            "name must start with a ascii alphabetic character",
        ));
    }

    // Must only contain a-zA-Z0-9_-
    if !value
        .chars()
        .all(|char| char.is_alphanumeric() || NAME_SEPARATORS.contains(&char))
    {
        return Err(garde::Error::new(
            "name must only contain alpha numeric values and _ or -",
        ));
    }

    // Must not end with - or _
    if value.ends_with(NAME_SEPARATORS) {
        return Err(garde::Error::new("name must not end with _ or -"));
    }

    Ok(())
}

impl Validate for ActionMap {
    type Context = ();

    fn validate_into(&self, ctx: &(), mut parent: &mut dyn FnMut() -> Path, report: &mut Report) {
        for (key, value) in self.0.iter() {
            let mut path = garde::util::nested_path!(parent, key);
            value.validate_into(ctx, &mut path, report);
        }
    }
}

impl PathComponentKind for ActionId {
    fn component_kind() -> Kind {
        Kind::Key
    }
}
