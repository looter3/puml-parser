use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::puml::code_generators::code_generator::{CodeGenerator, DestinationLanguage};

use crate::puml::core_parser::class::{Class, Field, Method};
use crate::puml::core_parser::interface::Interface;
use crate::puml::core_parser::types::Type;

pub struct JavaCodeGenerator;

impl CodeGenerator for JavaCodeGenerator {

    fn generate_source(&self, mut types: HashMap<String, Box<dyn Type>>) -> HashMap<String, String> {

        let start = Instant::now();

        let mut source_code = HashMap::new();

        types.iter_mut().for_each(|(type_name, _type)| {

            // Generate source code for the given type
            let type_src = _type.generate_source_code(DestinationLanguage::JAVA(JavaCodeGenerator));
            source_code.insert(type_name.to_string(), type_src);
        });
        let duration = start.elapsed();
        println!("Java - generate_source() time: {:?}", duration);
        return source_code;
    }

    fn generate_class(&self, class: &Class) -> String {
        let mut class_source = String::new();

        class_source.push_str(self.generate_class_signature(class.name(), class.extended_class(), class.interface()).as_str());

        class_source.push_str(self.generate_fields(class.fields()).as_str());

        // Add a separating line if there are both fields and methods
        /*
        if !class.fields().is_empty() && !class.methods().is_empty() {
            class_source.push_str("\n");
        }
         */

        class_source.push_str(self.generate_methods(class.methods()).as_str());

        // Close class body
        class_source.push_str("}");

        return class_source;
    }

    fn generate_class_signature(&self, class_name: &str, ext: &str, iface: &str) -> String {
        // Generate class signature
        let mut signature = format!("public class {}", class_name);

        // Append "extends ..." if an extended class is specified
        if !ext.is_empty() {
            signature.push_str(format!(" extends {}", ext).as_str());
        }

        // Append "implements ..." if an interface is specified
        if !iface.is_empty() {
            signature.push_str(format!(" implements {}", iface).as_str());
        }

        // Open class body
        signature.push_str(" {\n");

        return signature;
    }

    fn generate_fields(&self, fields: &HashSet<Field>) -> String {

        let mut fields_src = String::new();
        // Generate fields
        fields.iter().for_each(|field| {
            fields_src.push_str(format!("    {} {} {};\n",
                                        field.access_modifier(),
                                        field.attr_type(),
                                        field.name()).as_str());
        });

        return fields_src;
    }

    fn generate_methods(&self, methods: &HashSet<Method>) -> String {

        let mut methods_src = String::new();

        // Generate methods
        methods.iter().for_each(|method| {
            methods_src.push_str(format!("    {} {} {}({}) {{\n        // TODO: implement\n    }}\n",
                                         method.access_modifier(),
                                         method.return_type(),
                                         method.name(),
                                         method.to_string()
            ).as_str());
        });

        return methods_src;
    }

    fn generate_interface(&self, interface: &Interface) -> String {

        let mut iface_source = String::new();
        iface_source.push_str(format!("public interface {}{}", interface.name(), " {\n").as_str());
        // TODO add constants not variables
        iface_source.push_str(self.generate_fields(interface.constants()).as_str());
        iface_source.push_str(self.generate_methods(interface.methods()).as_str());

        // Close class body
        iface_source.push_str("}");

        return iface_source;
    }
}