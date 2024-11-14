use std::collections::HashMap;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use crate::puml::core_parser::regex_constants::{FIELD_REGEX, CLASS_REGEX, INTERFACE_REGEX, METHOD_REGEX, PARENT_CLASS_REGEX, REGEX_ERROR_MESSAGE, PARAMETER_REGEX};

// Global static variable holding the singleton instance
lazy_static! {
     // wrapped in a Mutex for thread-safety
    //pub static ref REGEX_SINGLETON: Mutex<RegexHolder> = Mutex::new(RegexHolder::new());
    pub static ref REGEX_SINGLETON: RegexHolder = RegexHolder::new();// as for now we don't need thread-safety
}

pub struct RegexHolder {
    regexes: HashMap<ClassRegex, Regex>
}

impl RegexHolder {
    fn new() -> Self {
        Self { regexes: get_all_regexes() }
    }

    pub fn regexes(&self) -> &HashMap<ClassRegex, Regex> {
        &self.regexes
    }
}

fn get_all_regexes() -> HashMap<ClassRegex, Regex> {

    let start = Instant::now();

    let mut map = HashMap::new();

    // Compile each regex only once and store directly in the HashMap
    map.insert(ClassRegex::CLASS, Regex::new(CLASS_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(ClassRegex::PARENT, Regex::new(PARENT_CLASS_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(ClassRegex::INTERFACE, Regex::new(INTERFACE_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(ClassRegex::FIELD, Regex::new(FIELD_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(ClassRegex::METHOD, Regex::new(METHOD_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(ClassRegex::PARAMETER, Regex::new(PARAMETER_REGEX).expect(REGEX_ERROR_MESSAGE));

    println!("Regexes compilation took {:?}", start.elapsed());

    map
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ClassRegex {
    CLASS,
    PARENT,
    INTERFACE,
    FIELD,
    METHOD,
    PARAMETER
}