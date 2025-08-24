use crate::plugin::{ActionId, ActionMap};
use garde::{
    Path, Report, Validate,
    error::{Kind, PathComponentKind},
};

/// Separators allowed in names
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

/// Validates that a string is a valid color value supports:
/// - hex
/// - rgb/rgba
/// - hsl/hsla
///
/// Does not check for named colors, we don't really want those anyway
/// as they aren't really useful
pub fn validate_color(value: &str, _context: &()) -> garde::Result {
    let value = value.trim().to_lowercase();

    // Hex
    if value.starts_with('#') {
        return validate_hex_color(&value);
    }

    // RGB
    if value.starts_with("rgb(") {
        return validate_rgb_color(&value);
    }

    // RGBA
    if value.starts_with("rgba(") {
        return validate_rgba_color(&value);
    }

    // HSL
    if value.starts_with("hsl(") {
        return validate_hsl_color(&value);
    }

    // HSLA
    if value.starts_with("hsla(") {
        return validate_hsla_color(&value);
    }

    Err(garde::Error::new("invalid color value"))
}

/// Validate a hex color
fn validate_hex_color(value: &str) -> garde::Result {
    let value = value
        .strip_prefix('#')
        .ok_or_else(|| garde::Error::new("hex color must start with #"))?;

    match value.len() {
        3 | 4 | 6 | 8 => {}
        _ => {
            return Err(garde::Error::new(
                "hex color must be 3, 4, 6, or 8 hex digits",
            ));
        }
    }

    if !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(garde::Error::new("hex color contains invalid characters"));
    }

    Ok(())
}

/// Validate a rgb() color
fn validate_rgb_color(value: &str) -> garde::Result {
    // Strip opening
    let value = value
        .strip_prefix("rgb(")
        .ok_or_else(|| garde::Error::new("rgb color must start with rgb("))?;

    // Strip closing
    let value = value
        .strip_suffix(")")
        .ok_or_else(|| garde::Error::new("unclosed rgb color"))?;

    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

    if parts.len() != 3 {
        return Err(garde::Error::new("invalid rgb color"));
    }

    for part in parts {
        parse_rgb_component(part)?;
    }

    Ok(())
}

/// Validate a rgba() color
fn validate_rgba_color(value: &str) -> garde::Result {
    // Strip opening
    let value = value
        .strip_prefix("rgba(")
        .ok_or_else(|| garde::Error::new("rgba color must start with rgba("))?;

    // Strip closing
    let value = value
        .strip_suffix(")")
        .ok_or_else(|| garde::Error::new("unclosed rgba color"))?;

    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

    if parts.len() != 4 {
        return Err(garde::Error::new("invalid rgba color"));
    }

    // RGB components
    for part in &parts[..3] {
        parse_rgb_component(part)?;
    }

    // Alpha component
    parse_alpha(parts[3])?;

    Ok(())
}

/// Validate a hsl() color
fn validate_hsl_color(value: &str) -> garde::Result {
    // Strip opening
    let value = value
        .strip_prefix("hsl(")
        .ok_or_else(|| garde::Error::new("hsl color must start with hsl("))?;

    // Strip closing
    let value = value
        .strip_suffix(")")
        .ok_or_else(|| garde::Error::new("unclosed hsl color"))?;

    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

    if parts.len() != 3 {
        return Err(garde::Error::new("invalid hsl color"));
    }

    parse_hue(parts[0])?;
    parse_percentage(parts[1])?;
    parse_percentage(parts[2])?;

    Ok(())
}

/// Validate a hsla() color
fn validate_hsla_color(value: &str) -> garde::Result {
    // Strip opening
    let value = value
        .strip_prefix("hsla(")
        .ok_or_else(|| garde::Error::new("hsla color must start with hsla("))?;

    // Strip closing
    let value = value
        .strip_suffix(")")
        .ok_or_else(|| garde::Error::new("unclosed hsla color"))?;

    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

    if parts.len() != 4 {
        return Err(garde::Error::new("invalid hsla color"));
    }

    parse_hue(parts[0])?;
    parse_percentage(parts[1])?;
    parse_percentage(parts[2])?;
    parse_alpha(parts[3])?;

    Ok(())
}

/// Parse an RGB component (0–255 or 0–100%)
fn parse_rgb_component(s: &str) -> garde::Result {
    if s.ends_with('%') {
        return parse_percentage(s);
    }

    let v: u16 = s.parse().map_err(|_| garde::Error::new("invalid number"))?;
    if v > 255 {
        return Err(garde::Error::new("rgb exceeded 255 bound"));
    }

    Ok(())
}

/// Parse an alpha channel (0–1)
fn parse_alpha(s: &str) -> garde::Result {
    if s.parse::<f64>()
        .is_ok_and(|value| (0.0..=1.0).contains(&value))
    {
        return Ok(());
    }

    Err(garde::Error::new("invalid alpha"))
}

/// Parse hue (0–360)
fn parse_hue(s: &str) -> garde::Result {
    let value: u16 = s.parse().map_err(|_| garde::Error::new("invalid hue"))?;
    if value > 360 {
        return Err(garde::Error::new("hue must not be greater than 360"));
    }

    Ok(())
}

/// Parse percentage (0–100%)
fn parse_percentage(s: &str) -> garde::Result {
    let number = s
        .strip_suffix('%')
        .ok_or_else(|| garde::Error::new("missing % sign"))?;

    let value: u8 = number
        .parse()
        .map_err(|_| garde::Error::new("invalid percent"))?;

    if value > 100 {
        return Err(garde::Error::new("percent > 100"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_id_allows_simple_valid_id() {
        assert!(validate_id("plugin.test", &()).is_ok());
        assert!(validate_id("abc.def123", &()).is_ok());
        assert!(validate_id("abc-def.ghi_jkl", &()).is_ok());
    }

    #[test]
    fn validate_id_fails_if_segment_does_not_start_with_letter() {
        assert!(validate_id("1plugin.test", &()).is_err());
        assert!(validate_id("plugin.1test", &()).is_err());
        assert!(validate_id(".test", &()).is_err());
    }

    #[test]
    fn validate_id_fails_if_segment_contains_invalid_characters() {
        assert!(validate_id("plugin.te$t", &()).is_err());
        assert!(validate_id("plugin.te st", &()).is_err());
        assert!(validate_id("plugin.te+st", &()).is_err());
    }

    #[test]
    fn validate_id_fails_if_segment_ends_with_separator() {
        assert!(validate_id("plugin.test_", &()).is_err());
        assert!(validate_id("plugin.test-", &()).is_err());
        assert!(validate_id("abc_.def", &()).is_err());
    }

    #[test]
    fn validate_name_allows_valid_name() {
        assert!(validate_name("ActionName", &()).is_ok());
        assert!(validate_name("my_action-1", &()).is_ok());
    }

    #[test]
    fn validate_name_fails_if_not_starting_with_letter() {
        assert!(validate_name("1Action", &()).is_err());
        assert!(validate_name("_Action", &()).is_err());
        assert!(validate_name("-Action", &()).is_err());
    }

    #[test]
    fn validate_name_fails_on_invalid_characters() {
        assert!(validate_name("Action!", &()).is_err());
        assert!(validate_name("Action Name", &()).is_err());
        assert!(validate_name("Action@", &()).is_err());
    }

    #[test]
    fn validate_name_fails_if_ends_with_separator() {
        assert!(validate_name("Action_", &()).is_err());
        assert!(validate_name("Action-", &()).is_err());
    }

    fn color_ok(value: &str) {
        assert!(
            validate_color(value, &()).is_ok(),
            "Expected OK for {value}"
        );
    }

    fn color_err(value: &str) {
        assert!(
            validate_color(value, &()).is_err(),
            "Expected ERR for {value}"
        );
    }

    #[test]
    fn test_valid_hex_colors() {
        color_ok("#fff");
        color_ok("#ffff");
        color_ok("#ffffff");
        color_ok("#ffffffff");
        color_ok("#ABC"); // uppercase allowed
        color_ok("  #123456  "); // trimming
    }

    #[test]
    fn test_invalid_hex_colors() {
        color_err("fff"); // missing #
        color_err("#ff"); // wrong length
        color_err("#fffff"); // wrong length
        color_err("#ggg"); // invalid hex digit
    }

    #[test]
    fn test_valid_rgb_colors() {
        color_ok("rgb(0,0,0)");
        color_ok("rgb(255, 255, 255)");
        color_ok("rgb(50%, 20%, 100%)");
    }

    #[test]
    fn test_invalid_rgb_colors() {
        color_err("rgb()");
        color_err("rgb(255,255)"); // not enough parts
        color_err("rgb(255,255,255,0)"); // too many parts
        color_err("rgb(300,0,0)"); // out of range
    }

    #[test]
    fn test_valid_rgba_colors() {
        color_ok("rgba(0,0,0,0)");
        color_ok("rgba(255,255,255,1)");
        color_ok("rgba(100, 150, 200, 0.5)");
        color_ok("rgba(10%,20%,30%,0.75)");
    }

    #[test]
    fn test_invalid_rgba_colors() {
        color_err("rgba(255,255,255)"); // missing alpha
        color_err("rgba(255,255,255,1,0)"); // too many parts
        color_err("rgba(255,255,255,2)"); // alpha > 1
        color_err("rgba(255,255,255,-0.1)"); // alpha < 0
    }

    #[test]
    fn test_valid_hsl_colors() {
        color_ok("hsl(0,0%,0%)");
        color_ok("hsl(360,100%,50%)");
        color_ok("hsl(180, 50%, 25%)");
    }

    #[test]
    fn test_invalid_hsl_colors() {
        color_err("hsl()");
        color_err("hsl(361,50%,50%)"); // hue too high
        color_err("hsl(180,101%,50%)"); // percent > 100
        color_err("hsl(180,50,50)"); // missing %
    }

    #[test]
    fn test_valid_hsla_colors() {
        color_ok("hsla(0,0%,0%,0)");
        color_ok("hsla(360,100%,50%,1)");
        color_ok("hsla(180, 50%, 25%, 0.75)");
    }

    #[test]
    fn test_invalid_hsla_colors() {
        color_err("hsla(180,50%,50%)"); // missing alpha
        color_err("hsla(180,50%,50%,2)"); // alpha too big
        color_err("hsla(361,50%,50%,0.5)"); // hue too big
        color_err("hsla(180,50,50%,0.5)"); // missing % in second arg
    }

    #[test]
    fn test_invalid_general_cases() {
        color_err("blue"); // named colors not supported
        color_err(""); // empty string
        color_err("123"); // junk input
    }
}
