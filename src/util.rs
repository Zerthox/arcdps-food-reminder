use serde::de::DeserializeOwned;

/// Parses JSONC from an input string.
pub fn parse_jsonc<T>(input: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    jsonc_parser::parse_to_serde_value(input)
        .ok()
        .and_then(|value| value)
        .and_then(|value| serde_json::from_value(value).ok())
}

/// Adjusts the alpha value of a color.
// TODO: for some reason rust complains here even if this is used?
#[allow(dead_code)]
pub fn with_alpha(mut color: [f32; 4], alpha: f32) -> [f32; 4] {
    color[3] = alpha;
    color
}
