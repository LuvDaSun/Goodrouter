use once_cell::sync::Lazy;
use regex::Regex;

pub mod template_pairs;
pub mod template_parts;

pub static TEMPLATE_PLACEHOLDER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{(.*?)\}").unwrap());
