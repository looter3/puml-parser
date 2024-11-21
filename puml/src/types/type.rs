use crate::code_generators::code_generator::{CodeGenerator, DestinationLanguage};
use crate::types::class::{Class, Field, Method};
use crate::types::interface::Interface;
use std::any::Any;

pub trait Type: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any; // Add for mutability
    fn add_member(&mut self, member: Member);
    fn generate_source_code(&self, destination_language: DestinationLanguage) -> String;
}

impl Type for Class {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add_member(&mut self, member: Member) {
        match member {
            Member::METHOD(method) => {
                self.add_method(method);
            }
            Member::FIELD(field) => {
                self.add_field(field);
            }
        }
    }

    fn generate_source_code(&self, destination_language: DestinationLanguage) -> String {
        match destination_language {
            DestinationLanguage::JAVA(generator) => generator.generate_class(&self),
        }
    }
}

impl Type for Interface {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add_member(&mut self, member: Member) {
        match member {
            Member::METHOD(method) => {
                self.add_method(method);
            }
            Member::FIELD(constant) => {
                self.add_constant(constant);
            }
        }
    }

    fn generate_source_code(&self, destination_language: DestinationLanguage) -> String {
        match destination_language {
            DestinationLanguage::JAVA(generator) => generator.generate_interface(&self),
        }
    }
}

pub enum Member {
    METHOD(Method),
    FIELD(Field),
}
