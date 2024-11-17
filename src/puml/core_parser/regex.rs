use once_cell::sync::Lazy;

use regex::Regex;

use crate::puml::core_parser::regex_constants::{FIELD_PATTERN, CLASS_PATTERN, IMPL_INTERFACE_PATTERN, METHOD_PATTERN, PARENT_CLASS_PATTERN, REGEX_ERROR_MESSAGE, PARAMETER_PATTERN, INTERFACE_PATTERN};

// Static Regex instances
static CLASS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(CLASS_PATTERN).expect(REGEX_ERROR_MESSAGE));
static PARENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(PARENT_CLASS_PATTERN).expect(REGEX_ERROR_MESSAGE));
static IMPL_INTERFACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(IMPL_INTERFACE_PATTERN).expect(REGEX_ERROR_MESSAGE));
static FIELD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(FIELD_PATTERN).expect(REGEX_ERROR_MESSAGE));
static METHOD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(METHOD_PATTERN).expect(REGEX_ERROR_MESSAGE));
static PARAMETER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(PARAMETER_PATTERN).expect(REGEX_ERROR_MESSAGE));
static INTERFACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(INTERFACE_PATTERN).expect(REGEX_ERROR_MESSAGE));

#[allow(non_camel_case_types)]
pub enum PUMLRegex {
    // Class
    CLASS,
    PARENT,
    IMPL_INTERFACE,
    FIELD,
    METHOD,
    PARAMETER,
    // Interface
    INTERFACE,
}

impl PUMLRegex {
    /// Get the associated Regex for the enum variant
    pub fn get_regex(&self) -> &Regex {
        match self {
            PUMLRegex::CLASS => &*CLASS_REGEX,
            PUMLRegex::PARENT => &*PARENT_REGEX,
            PUMLRegex::IMPL_INTERFACE => &*IMPL_INTERFACE_REGEX,
            PUMLRegex::FIELD => &*FIELD_REGEX,
            PUMLRegex::METHOD => &*METHOD_REGEX,
            PUMLRegex::PARAMETER => &*PARAMETER_REGEX,
            PUMLRegex::INTERFACE => &*INTERFACE_REGEX,
        }
    }
}