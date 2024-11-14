use std::collections::HashMap;
use std::fmt::Write;
use std::time::Instant;

use crate::puml::core_parser::class::Class;

pub fn generate_java_code(mut classes: HashMap<String, Class>) -> HashMap<String, String> {

    let start = Instant::now();

    let mut classes_source = HashMap::new();

    classes.iter_mut().for_each(|(class_name, class)| {
        generate_class(&mut classes_source, class_name, class);
    });
    let duration = start.elapsed();
    println!("generate_java_code() time: {:?}", duration);
    return classes_source;
}

fn generate_class(classes_source: &mut HashMap<String, String>, class_name: &str, class: &mut Class) {
    let mut class_source = String::new();

    generate_class_signature(class_name, class, &mut class_source);

    generate_fields(class, &mut class_source);

    // Add a separating line if there are both fields and methods
    if !class.fields().is_empty() && !class.methods().is_empty() {
        class_source.push_str("\n");
    }

    generate_methods(class, &mut class_source);

    // Close class body
    class_source.push_str("}");

    // Insert into classes_source map
    classes_source.insert(class_name.to_string(), class_source);
}

fn generate_methods(class: &mut Class, class_source: &mut String) {
    // Generate methods
    class.methods().iter().for_each(|method| {
        let _ = write!(
            class_source,
            "    {} {} {}({}) {{\n        // TODO: implement\n    }}\n",
            method.access_modifier(),
            method.return_type(),
            method.name(),
            method.to_string()
        );
    });
}

fn generate_fields(class: &mut Class, class_source: &mut String) {
    // Generate fields
    class.fields().iter().for_each(|field| {
        let _ = write!(
            class_source,
            "    {} {} {};\n",
            field.access_modifier(),
            field.attr_type(),
            field.name()
        );
    });
}

fn generate_class_signature(class_name: &str, class: &mut Class, class_source: &mut String) {
    // Generate class signature
    write!(class_source, "public class {}", class_name).unwrap();

    // Append "extends ..." if an extended class is specified
    let ext = class.extended_class();
    if !ext.is_empty() {
        write!(class_source, " extends {}", ext).unwrap();
    }

    // Append "implements ..." if an interface is specified
    let iface = class.interface();
    if !iface.is_empty() {
        write!(class_source, " implements {}", iface).unwrap();
    }

    // Open class body
    class_source.push_str(" {\n");
}