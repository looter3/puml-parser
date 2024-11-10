use std::collections::HashMap;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

use crate::puml::core_parser::regex_constants::{ATTRIBUTE_REGEX, CLASS_REGEX, INTERFACE_REGEX, METHOD_REGEX, PARENT_CLASS_REGEX, REGEX_ERROR_MESSAGE};

// Global static variable holding the singleton instance
lazy_static! {
     // wrapped in a Mutex for thread-safety
    //pub static ref REGEX_SINGLETON: Mutex<RegexHolder> = Mutex::new(RegexHolder::new());W
    pub static ref REGEX_SINGLETON: RegexHolder = RegexHolder::new();// as for now we don't need thread-safety
}

pub struct RegexHolder {
    regexes: HashMap<RegexType, Regex>
}

impl RegexHolder {
    fn new() -> Self {
        Self { regexes: get_all_regexes() }
    }

    pub fn regexes(&self) -> &HashMap<RegexType, Regex> {
        &self.regexes
    }
}

fn get_all_regexes() -> HashMap<RegexType, Regex> {

    let start = Instant::now();

    let mut map = HashMap::new();

    // Compile each regex only once and store directly in the HashMap
    map.insert(RegexType::CLASS, Regex::new(CLASS_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(RegexType::PARENT, Regex::new(PARENT_CLASS_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(RegexType::INTERFACE, Regex::new(INTERFACE_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(RegexType::ATTRIBUTE, Regex::new(ATTRIBUTE_REGEX).expect(REGEX_ERROR_MESSAGE));
    map.insert(RegexType::METHOD, Regex::new(METHOD_REGEX).expect(REGEX_ERROR_MESSAGE));

    println!("Regexes compilation took {:?}", start.elapsed());

    map
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum RegexType {
    CLASS,
    PARENT,
    INTERFACE,
    ATTRIBUTE,
    METHOD,
}