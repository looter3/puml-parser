use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;
use crate::puml::code_generators::code_generator::{SourceCodeGenerator, SourceCodeStrategy};
use crate::puml::common::constants::{CH_PRIVATE, CH_PROTECTED, CH_PUBLIC, EMPTY_STRING};

use crate::puml::core_parser::class::{AccessModifier, Class, Field, Method};
use crate::puml::core_parser::regex::{REGEX_SINGLETON, ClassRegex};

pub fn parse(file: File, source_code_strategy: SourceCodeStrategy) -> HashMap<String, String> {

    let start = Instant::now();

    let parsing_res = parse_puml(file);

    let mut res = HashMap::new();

    if parsing_res.is_ok() {
        let parsed = parsing_res.unwrap();

        let src_gen = SourceCodeGenerator::new(source_code_strategy);
        res = src_gen.generate_source_code(parsed);
    }
    let duration = start.elapsed();
    println!("Computation time: {:?}", duration);

    res
}

fn parse_puml(file: File) -> Result<HashMap<String, Class>, String> {

    let start = Instant::now();

    let reader = BufReader::new(file);
    let mut current_class = String::new();
    let mut classes = HashMap::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        match line {
            Ok(l) => parse_line(&mut current_class, &mut classes, l),
            Err(e) => return Err(format!("Error reading line: {}", e)),
        }
    }
    let method_duration = start.elapsed();
    println!("parse_puml() time: {:?}", method_duration);

    // Return the classes if everything went well
    Ok(classes)
}

fn parse_line(mut current_class: &mut String, classes: &mut HashMap<String, Class>, l: String) {

    let current_line = l.trim().to_string();

    let output = process_line(current_line, &mut current_class);

    if let Some(output) = output {

        match output {
            ProcessLineOutput::CLASS(class) => {classes.insert(class.0, class.1);},
            ProcessLineOutput::MEMBER(member) => {on_member_found(&mut current_class, classes, member);},
        }
    }
}

fn on_member_found(current_class: &mut String, classes: &mut HashMap<String, Class>, member: Member) {
    match member {
        Member::METHOD(method) => {
            classes.get_mut(&*current_class).map(|class| {
                class.add_method(method);
            });
        }
        Member::FIELD(field) => {
            classes.get_mut(&*current_class).map(|class| {
                class.add_field(field);
            });
        }
    }
}

fn process_line(line: String, current_class: &mut String/*, classes: &mut HashMap<String, Class>*/) -> Option<ProcessLineOutput> {

    let class = extract_class_definition(&line, current_class);
    let member = extract_member(&line);

    if let Some(class) = class {
        return Some(ProcessLineOutput::CLASS(class));
    }
    if let Some(member) = member {
        return Some(ProcessLineOutput::MEMBER(member));
    }

    return None;
}

enum ProcessLineOutput {
    CLASS((String, Class)),
    MEMBER(Member)
}

fn extract_class_definition(line: &String, current_class: &mut String/*, classes: &HashMap<String, Class>*/) -> Option<(String, Class)> {

    let regex_map= REGEX_SINGLETON.regexes();

    // Detect class definition
    let class_entry = extract_captures(&regex_map[&ClassRegex::CLASS], line, vec![1])
        .map(|vec| {

            let class_name = vec.get(0).unwrap();
            current_class.clear();
            current_class.push_str(class_name);
            let mut class = Class::new();

            let parent = extract_parent(&line);
            let interface = extract_interface(&line);

            if let Some(parent) = parent {
                class.set_extended_class(parent);
            }
            if let Some(interface) = interface {
                class.set_interface(interface);
            }

            return (current_class.clone(), class);
        });

    return class_entry;
}

fn extract_parent(line: &String/*, current_class: &mut String, classes: &mut HashMap<String, Class>*/) -> Option<String> {

    let regex_map = REGEX_SINGLETON.regexes();

    // Detect parent class
    let parent = extract_captures(&regex_map[&ClassRegex::PARENT], line, vec![1])
        .map(|vec| {
            let parent = vec.get(0).unwrap();
            return parent.to_string();
        });

    return parent;
}

fn extract_interface(line: &String/*, current_class: &mut String, classes: &mut HashMap<String, Class>*/) -> Option<String> {

    let regex_map = REGEX_SINGLETON.regexes();

    // Detect interface
    let interface = extract_captures(&regex_map[&ClassRegex::INTERFACE], line, vec![1])
        .map(|vec| {
            let interface = vec.get(0).unwrap();
            return interface.to_string();
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

enum Member {
    METHOD(Method),
    FIELD(Field)
}

fn extract_access_modifier(line: &String) -> String {
    // Detect access modifier
    let access_modifier_string = line.chars().next()
        .and_then(|first_char| get_access_modifier(first_char))
        .map(|modifier| match modifier {
            AccessModifier::PUBLIC(s) | AccessModifier::PRIVATE(s) | AccessModifier::PROTECTED(s) => s,
        })
        .unwrap_or(EMPTY_STRING.to_string());

    return access_modifier_string;
}

fn extract_method(line: &String/*, current_class: &mut String, classes: &mut HashMap<String, Class>*/, access_modifier_string: &String) -> Option<Method> {

    let regex_map = REGEX_SINGLETON.regexes();

    let method = extract_captures(&regex_map[&ClassRegex::METHOD], line.as_str(), vec![2, 3, 4])
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
                parameters
            );
        });

    return method;
}

fn extract_parameters(raw_param: &str) -> BTreeMap<String, String> {
    // Assuming REGEX_SINGLETON.regexes() returns a map of regex patterns,
    // and we're accessing the specific regex for parameters.
    let regex_map = REGEX_SINGLETON.regexes();

    // Create a HashMap to store the extracted parameters
    let mut params = BTreeMap::new();

    // Split the raw_param string by commas and trim whitespace from each part
    raw_param.split(',')
        .map(|s| s.trim()) // Trim whitespace
        .for_each(|parameter| {
            // Use the regex to extract the type and name
            if let Some(captures) = extract_captures(&regex_map[&ClassRegex::PARAMETER], parameter, vec![1, 2]) {
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

    let regex_map = REGEX_SINGLETON.regexes();

    // Extract attribute
    let field = extract_captures(&regex_map[&ClassRegex::FIELD], line.as_str(), vec![1, 2])
        .map(|vec| {
            let _type = vec.get(0).unwrap();
            let _name = vec.get(1).unwrap();
            return Field::new(access_modifier.to_string(), _name.to_string(), _type.to_string());
        });

    return field;
}

// Helper function to extract two capture groups (used for methods and attributes)
fn extract_captures<'a>(regex: &Regex, text: &'a str, groups: Vec<usize>) -> Option<Vec<&'a str>> {
    // Collect the captures from each group, filtering out None values
    let captures: Option<Vec<&'a str>> = groups.iter()
        .map(|&group| {
            regex.captures(text)
                .and_then(|cap| {
                    cap.get(group).map(|m| {
                        m.as_str()
                    })
                })
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