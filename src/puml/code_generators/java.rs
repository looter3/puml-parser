use std::collections::HashMap;
use std::time::Instant;

use crate::puml::core_parser::class::Class;

pub fn generate_java_code(mut classes: HashMap<String, Class>) -> HashMap<String, String> {

    let start = Instant::now();

    let mut classes_source = HashMap::new();

    classes.iter_mut().for_each(|(class_name, class)| {

        let mut class_source = String::new();

        // class signature
        let mut signature = format!("public class {}", class_name);
        // extended class
        if !class.extended_class().is_empty() {
            signature.push_str("extends ");
            signature.push_str(class.extended_class());
            signature.push_str(" ");
        }
        // interface
        if !class.interface().is_empty() {
            signature.push_str("implements ");
            signature.push_str(class.interface());
            signature.push_str(" ");
        }
        class_source.push_str(signature.as_str());
        // open curly bracket
        class_source.push_str("{\n");

        // attributes
        class.attributes().iter_mut().for_each(|attribute| {
            let attr = format!("    {} {} {}", attribute.access_modifier(), attribute.attr_type(), attribute.name());
            class_source.push_str(attr.as_str());
            class_source.push_str(";\n");
        });

        // put empty line
        class_source.push_str("\n");

        // methods
        class.methods().iter_mut().for_each(|method| {
            let meth = format!("    {} {} {}() {{\n", method.access_modifier(), method.return_type(), method.name());
            class_source.push_str(meth.as_str());
            class_source.push_str("        // TODO: implement\n");
            class_source.push_str("    }\n");
        });

        // add closing bracket to class
        class_source.push_str("}");

        // add to map
        classes_source.insert(class_name.to_string(), class_source);
        //classes_source.push(class_source);
    });
    let duration = start.elapsed();
    println!("generate_java_code() time: {:?}", duration);
    return classes_source;
}