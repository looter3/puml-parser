use std::collections::{HashMap, HashSet};

use crate::code_generators::java::JavaCodeGenerator;
use crate::types::class::{Class, Field, Method};
use crate::types::interface::Interface;
use crate::types::r#type::Type;

pub trait CodeGenerator {
    fn generate_source(&self, types: HashMap<String, Box<dyn Type>>) -> HashMap<String, String>;
    // Class
    fn generate_class(&self, class: &Class) -> String;
    fn generate_class_signature(&self, class_name: &str, ext: &str, iface: &str) -> String;
    fn generate_fields(&self, fields: &HashSet<Field>) -> String;
    fn generate_methods(&self, methods: &HashSet<Method>) -> String;
    //fn generate_interface_signature(interface: &Interface) -> String;
    // Interface
    fn generate_interface(&self, interface: &Interface) -> String;
}

pub enum DestinationLanguage {
    JAVA(JavaCodeGenerator),
}
