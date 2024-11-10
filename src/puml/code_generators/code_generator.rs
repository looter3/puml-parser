use std::collections::HashMap;
use crate::puml::core_parser::class::Class;

/**
We use strategy pattern to parse in the desired destination language
*/
pub type SourceCodeStrategy = fn(classes: HashMap<String, Class>) -> HashMap<String, String>;

pub struct SourceCodeGenerator {
    source_code_strategy: SourceCodeStrategy
}

impl SourceCodeGenerator {
    pub fn new(source_code_strategy: SourceCodeStrategy) -> Self {
        Self { source_code_strategy }
    }

    /**
    Resulting map  has key = to class name; value = source code
    */
    pub fn generate_source_code(&self, classes: HashMap<String, Class>) -> HashMap<String, String> {
        (self.source_code_strategy)(classes)
    }
}