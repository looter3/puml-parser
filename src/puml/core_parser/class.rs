use std::collections::{BTreeMap, HashSet};
use crate::puml::common::constants::EMPTY_STRING;

pub struct Class {
    fields: HashSet<Field>,
    extended_class: String,
    interface: String,
    methods: HashSet<Method>
}

impl Class {
    pub fn new() -> Self {
        Self {
            fields: HashSet::new(),
            extended_class: EMPTY_STRING.to_string(),
            interface: EMPTY_STRING.to_string(),
            methods: HashSet::new()
        }
    }

    pub fn add_method(&mut self, method: Method) {
        self.methods.insert(method);
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.insert(field);
    }

    // Getters
    pub fn fields(&mut self) -> &mut HashSet<Field> {
        &mut self.fields
    }
    pub fn extended_class(&self) -> &str {
        &self.extended_class
    }
    pub fn interface(&self) -> &str {
        &self.interface
    }
    pub fn methods(&self) -> &HashSet<Method> {
        &self.methods
    }

    // Setters
    pub fn set_fields(&mut self, attributes: HashSet<Field>) {
        self.fields = attributes;
    }
    pub fn set_extended_class(&mut self, extended_class: String) {
        self.extended_class = extended_class;
    }
    pub fn set_interface(&mut self, interface: String) {
        self.interface = interface;
    }
    pub fn set_methods(&mut self, methods: HashSet<Method>) {
        self.methods = methods;
    }

}

#[derive(Eq, Hash, PartialEq)]
pub struct Field {
    access_modifier: String,
    name: String,
    attr_type:String
}

impl Field {
    pub fn new(access_modifier: String, name: String, attr_type: String) -> Self {
        Self { access_modifier, name, attr_type }
    }

    // Getters
    pub fn access_modifier(&self) -> &str {
        &self.access_modifier
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn attr_type(&self) -> &str {
        &self.attr_type
    }

    // Setters
    pub fn set_access_modifier(&mut self, access_modifier: String) {
        self.access_modifier = access_modifier;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_attr_type(&mut self, attr_type: String) {
        self.attr_type = attr_type;
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Method {
    access_modifier: String,
    name: String,
    return_type: String,
    parameters: BTreeMap<String, String> // key is type or name and value... you get the point
}

impl Method {
    pub fn new(access_modifier: String, name: String, return_type: String, parameters: BTreeMap<String, String>) -> Self {
        Self { access_modifier, name, return_type, parameters }
    }

    pub fn to_string(&self) -> String {
        self.parameters
            .iter()
            .map(|(name, type_)| format!("{} {}", type_, name)) // Format each parameter
            .collect::<Vec<_>>() // Collect into a Vec of Strings
            .join(", ") // Join with ", " separator
    }

    // Getters
    pub fn access_modifier(&self) -> &str {
        &self.access_modifier
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }
    pub fn return_type(&self) -> &str {
        &self.return_type
    }

    // Setters
    pub fn set_access_modifier(&mut self, access_modifier: String) {
        self.access_modifier = access_modifier;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_return_type(&mut self, return_type: String) {
        self.return_type = return_type;
    }
    pub fn set_parameters(&mut self, parameters: BTreeMap<String, String>) {
        self.parameters = parameters;
    }
}

// Define an enum to represent the UML access modifiers with an associated string
#[derive(Debug)]
pub enum AccessModifier {
    PUBLIC(String),
    PRIVATE(String),
    PROTECTED(String),
}