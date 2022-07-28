use jsonc_parser::ParseOptions;
use serde::de::DeserializeOwned;

/// Parses JSONC from an input string.
pub fn parse_jsonc<T>(input: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    jsonc_parser::parse_to_serde_value(input, &ParseOptions::default())
        .ok()
        .and_then(|value| value)
        .and_then(|value| serde_json::from_value(value).ok())
}
