use std::collections::HashSet;
use crate::types::class::{Field, Method};

pub struct Interface {
    name: String,
    constants: HashSet<Field>,
    methods: HashSet<Method>,
}

impl Interface {
    pub fn new(name: String) -> Self {
        Self {
            name,
            constants: HashSet::new(),
            methods: HashSet::new(),
        }
    }

    pub fn add_method(&mut self, method: Method) {
        self.methods.insert(method);
    }
    pub fn add_constant(&mut self, field: Field) {
        self.constants.insert(field);
    }

    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn constants(&self) -> &HashSet<Field> {
        &self.constants
    }
    pub fn methods(&self) -> &HashSet<Method> {
        &self.methods
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_constants(&mut self, constants: HashSet<Field>) {
        self.constants = constants;
    }
    pub fn set_methods(&mut self, methods: HashSet<Method>) {
        self.methods = methods;
    }
}
