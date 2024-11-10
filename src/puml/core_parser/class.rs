use crate::puml::common::constants::EMPTY_STRING;

pub struct Class {
    attributes: Vec<Attribute>,
    extended_class: String,
    interface: String,
    methods: Vec<Method>
}

impl Class {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            extended_class: EMPTY_STRING.to_string(),
            interface: EMPTY_STRING.to_string(),
            methods: Vec::new()
        }
    }

    // Getters
    pub fn attributes(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }
    pub fn extended_class(&self) -> &str {
        &self.extended_class
    }
    pub fn interface(&self) -> &str {
        &self.interface
    }
    pub fn methods(&mut self) -> &mut Vec<Method> {
        &mut self.methods
    }

    // Setters
    pub fn set_attributes(&mut self, attributes: Vec<Attribute>) {
        self.attributes = attributes;
    }
    pub fn set_extended_class(&mut self, extended_class: String) {
        self.extended_class = extended_class;
    }
    pub fn set_interface(&mut self, interface: String) {
        self.interface = interface;
    }
    pub fn set_methods(&mut self, methods: Vec<Method>) {
        self.methods = methods;
    }
}

pub struct Attribute {
    access_modifier: String,
    name: String,
    attr_type:String
}

impl Attribute {
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

pub struct Method {
    access_modifier: String,
    name: String,
    return_type:String
}

impl Method {
    pub fn new(access_modifier: String, name: String, return_type: String) -> Self {
        Self { access_modifier, name, return_type }
    }

    // Setters
    pub fn access_modifier(&self) -> &str {
        &self.access_modifier
    }
    pub fn name(&self) -> &str {
        &self.name
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
}

// Define an enum to represent the UML access modifiers with an associated string
#[derive(Debug)]
pub enum AccessModifier {
    PUBLIC(String),
    PRIVATE(String),
    PROTECTED(String),
}