use crate::code_generators::code_generator::CodeGenerator;
use crate::common::constants::{CH_PRIVATE, CH_PROTECTED, CH_PUBLIC, EMPTY_STRING};
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::types::class::{AccessModifier, Class, Field, Method};
use crate::types::interface::Interface;
use crate::core_parser::regex::PUMLRegex;
use crate::types::r#type::{Member, Type};

pub fn parse(file: File, code_generator: Box<dyn CodeGenerator>) -> HashMap<String, String> {
    let start = Instant::now();

    let parsing_res = parse_puml(file);

    let mut res = HashMap::new();

    if parsing_res.is_ok() {
        let parsed = parsing_res.unwrap();

        // Generate source code in destination language
        res = code_generator.generate_source(parsed);
    }
    let duration = start.elapsed();
    println!("Computation time: {:?}", duration);

    res
}

fn parse_puml(file: File) -> Result<HashMap<String, Box<dyn Type>>, String> {
    let start = Instant::now();

    let reader = BufReader::new(file);
    let mut current_element = String::new();
    let mut source = HashMap::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        match line {
            Ok(l) => parse_line(l, &mut current_element, &mut source),
            Err(e) => return Err(format!("Error reading line: {}", e)),
        }
    }
    let method_duration = start.elapsed();
    println!("parse_puml() time: {:?}", method_duration);

    // Return the classes if everything went well
    Ok(source)
}

fn parse_line(
    line: String,
    mut current_element: &mut String,
    elements: &mut HashMap<String, Box<dyn Type>>,
) {
    let current_line = line.trim().to_string();

    let output = process_line(current_line, &mut current_element);

    if let Some(output) = output {
        match output {
            ProcessLineOutput::MEMBER(member) => {
                on_member_found(&mut current_element, elements, member);
            }
            ProcessLineOutput::TYPE(_type) => {
                elements.insert(_type.0, _type.1);
            }
        }
    }
}

/**
 *   Add member to Type
 */
fn on_member_found(
    current_element: &mut String,
    elements: &mut HashMap<String, Box<dyn Type>>,
    member: Member,
) {
    elements.get_mut(&*current_element).map(|_type| {
        _type.add_member(member);
    });
}

/**
 *  Process next line and returns an output
 */
fn process_line(line: String, current_element: &mut String) -> Option<ProcessLineOutput> {
    // TODO add support for all types
    let class = extract_class_definition(&line, current_element);
    let interface = extract_interface_definition(&line, current_element);
    let member = extract_member(&line);

    if let Some(class) = class {
        return Some(ProcessLineOutput::TYPE((
            class.0.to_string(),
            Box::new(class.1),
        )));
    }
    if let Some(iface) = interface {
        return Some(ProcessLineOutput::TYPE((
            iface.0.to_string(),
            Box::new(iface.1),
        )));
    }
    if let Some(member) = member {
        return Some(ProcessLineOutput::MEMBER(member));
    }

    return None;
}

enum ProcessLineOutput {
    TYPE((String, Box<dyn Type>)),
    MEMBER(Member),
}

fn extract_class_definition(
    line: &String,
    current_element: &mut String,
) -> Option<(String, Class)> {
    // Detect class definition
    let class_entry = extract_captures(&PUMLRegex::CLASS.get_regex(), line, vec![1]).map(|vec| {
        let class_name = vec.get(0).unwrap();
        current_element.clear();
        current_element.push_str(class_name);
        let mut class = Class::new(class_name.to_string());

        let parent = extract_parent(&line);
        let interface = extract_implemented_interface(&line);

        if let Some(parent) = parent {
            class.set_extended_class(parent);
        }
        if let Some(interface) = interface {
            class.set_interface(interface);
        }

        return (current_element.clone(), class);
    });

    return class_entry;
}

fn extract_parent(line: &String) -> Option<String> {
    // Detect parent class
    extract_hierarchy(line, &PUMLRegex::PARENT.get_regex())
}

fn extract_implemented_interface(line: &String) -> Option<String> {
    // Detect interface
    extract_hierarchy(line, &PUMLRegex::IMPL_INTERFACE.get_regex())
}

fn extract_hierarchy(line: &str, regex: &Regex) -> Option<String> {
    extract_captures(regex, line, vec![1])
        .and_then(|captures| captures.get(0).map(|s| s.to_string()))
}

fn extract_interface_definition(
    line: &String,
    current_element: &mut String,
) -> Option<(String, Interface)> {
    let interface = extract_captures(&PUMLRegex::INTERFACE.get_regex(), line, vec![1]).map(|vec| {
        let iface = vec.get(0).unwrap();
        current_element.clear();
        current_element.push_str(iface);
        let interface = Interface::new(iface.to_string());

        return (current_element.clone(), interface);
    });
    return interface;
}

fn extract_member(line: &String) -> Option<Member> {
    let access_modifier = extract_access_modifier(&line);

    if !access_modifier.is_empty() {
        let method = extract_method(&line, &access_modifier);
        let field = extract_field(&line, &access_modifier);

        if let Some(method) = method {
            return Some(Member::METHOD(method));
        }

        if let Some(field) = field {
            return Some(Member::FIELD(field));
        }
    }
    return None;
}

fn extract_access_modifier(line: &String) -> String {
    // Detect access modifier
    let access_modifier_string = line
        .chars()
        .next()
        .and_then(|first_char| get_access_modifier(first_char))
        .map(|modifier| match modifier {
            AccessModifier::PUBLIC(s)
            | AccessModifier::PRIVATE(s)
            | AccessModifier::PROTECTED(s) => s,
        })
        .unwrap_or(EMPTY_STRING.to_string());

    return access_modifier_string;
}

fn extract_method(line: &String, access_modifier_string: &String) -> Option<Method> {
    let method = extract_captures(&PUMLRegex::METHOD.get_regex(), line.as_str(), vec![2, 3, 4])
        .map(|vec| {
            let return_type = vec.get(0).unwrap();
            let method_name = vec.get(1).unwrap();

            // Collect the results into a HashMap
            let raw_parameters = vec.get(2).unwrap();
            let parameters = extract_parameters(raw_parameters);

            return Method::new(
                access_modifier_string.clone(),
                method_name.to_string(),
                return_type.to_string(),
                parameters,
            );
        });

    return method;
}

fn extract_parameters(raw_param: &str) -> BTreeMap<String, String> {
    // Create a HashMap to store the extracted parameters
    let mut params = BTreeMap::new();

    // Split the raw_param string by commas and trim whitespace from each part
    raw_param
        .split(',')
        .map(|s| s.trim()) // Trim whitespace
        .for_each(|parameter| {
            // Use the regex to extract the type and name
            if let Some(captures) =
                extract_captures(&PUMLRegex::PARAMETER.get_regex(), parameter, vec![1, 2])
            {
                // captures should contain type (Group 1) and name (Group 2)
                if let Some(_type) = captures.get(0) {
                    if let Some(_name) = captures.get(1) {
                        // Insert the type-name pair into the HashMap
                        params.insert(_name.to_string(), _type.to_string());
                    }
                }
            }
        });

    // Return the HashMap containing all parameters
    params
}

fn extract_field(line: &String, access_modifier: &String) -> Option<Field> {
    // Extract attribute
    let field =
        extract_captures(&PUMLRegex::FIELD.get_regex(), line.as_str(), vec![1, 2]).map(|vec| {
            let _type = vec.get(0).unwrap();
            let _name = vec.get(1).unwrap();
            return Field::new(
                access_modifier.to_string(),
                _name.to_string(),
                _type.to_string(),
            );
        });

    return field;
}

// Helper function to extract two capture groups (used for methods and attributes)
fn extract_captures<'a>(regex: &Regex, text: &'a str, groups: Vec<usize>) -> Option<Vec<&'a str>> {
    // Collect the captures from each group, filtering out None values
    let captures: Option<Vec<&'a str>> = groups
        .iter()
        .map(|&group| {
            regex
                .captures(text)
                .and_then(|cap| cap.get(group).map(|m| m.as_str()))
        })
        .collect();

    // Return the captured values only if all groups are found
    captures
}

// Function to convert the first character of a string into an access modifier with a string representation
fn get_access_modifier(c: char) -> Option<AccessModifier> {
    match c {
        CH_PUBLIC => Some(AccessModifier::PUBLIC("public".to_string())),
        CH_PRIVATE => Some(AccessModifier::PRIVATE("private".to_string())),
        CH_PROTECTED => Some(AccessModifier::PROTECTED("protected".to_string())),
        _ => None, // Handle invalid character
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::code_generators::java::JavaCodeGenerator;
    use crate::common::constants::TEST_DATA_PATH;
    use crate::core_parser::parser::parse;

    #[test]
    fn parse_e2e() {

        let path = format!("{}{}", TEST_DATA_PATH, "test.puml"); // Relative path to the test file
        let file = File::open(path.as_str()).expect(format!("Failed to read file: {}", path).as_str());
        parse(file, Box::new(JavaCodeGenerator));

        assert!(true);
    }
}