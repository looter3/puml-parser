use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;
use crate::puml::code_generators::code_generator::{SourceCodeGenerator, SourceCodeStrategy};
use crate::puml::common::constants::{CH_PRIVATE, CH_PROTECTED, CH_PUBLIC};

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
            Ok(l) => {
                let current_line = l.trim().to_string();
                process_line(current_line, &mut current_class, &mut classes);
            },
            Err(e) => return Err(format!("Error reading line: {}", e)),
        }
    }
    let method_duration = start.elapsed();
    println!("parse_puml() time: {:?}", method_duration);

    // Return the classes if everything went well
    Ok(classes)
}

fn process_line(line: String, current_class: &mut String, classes: &mut HashMap<String, Class>) {

    // let regex_mutex = REGEX_SINGLETON.lock().unwrap();

    let regex_map = REGEX_SINGLETON.regexes();

    extract_class(&line, current_class, classes, regex_map);

    extract_parent(&line, current_class, classes, regex_map);

    extract_interface(&line, current_class, classes, regex_map);

    extract_members(line, current_class, classes, regex_map);
}

fn extract_class(line: &String, current_class: &mut String, classes: &mut HashMap<String, Class>, regex_map: &HashMap<ClassRegex, Regex>) {
    // Detect class definition
    extract_captures(&regex_map[&ClassRegex::CLASS], line, vec![1])
        .map(|vec| {
            let class_name = vec.get(0).unwrap();
            current_class.clear();
            current_class.push_str(class_name);
            classes.insert(current_class.clone(), Class::new());
        });
}

fn extract_parent(line: &String, current_class: &mut String, classes: &mut HashMap<String, Class>, regex_map: &HashMap<ClassRegex, Regex>) {
    // Detect parent class
    extract_captures(&regex_map[&ClassRegex::PARENT], line, vec![1])
        .map(|vec| {
            let parent = vec.get(0).unwrap();
            classes.get_mut(current_class).map(|class| {
                class.set_extended_class(parent.to_string());
            });
        });
}

fn extract_interface(line: &String, current_class: &mut String, classes: &mut HashMap<String, Class>, regex_map: &HashMap<ClassRegex, Regex>) {
    // Detect interface
    extract_captures(&regex_map[&ClassRegex::INTERFACE], line, vec![1])
        .map(|vec| {
            let interface = vec.get(0).unwrap();
            classes.get_mut(current_class).map(|class| {
                class.set_interface(interface.to_string());
            });
        });
}

fn extract_members(line: String, current_class: &mut String, classes: &mut HashMap<String, Class>, regex_map: &HashMap<ClassRegex, Regex>) {

    if !current_class.is_empty() && (line.contains(CH_PRIVATE) || line.contains(CH_PUBLIC) || line.contains(CH_PROTECTED)) {

        // Detect access modifier
        let access_modifier_string = line.chars().next()
            .and_then(|first_char| get_access_modifier(first_char))
            .map(|modifier| match modifier {
                AccessModifier::PUBLIC(s) | AccessModifier::PRIVATE(s) | AccessModifier::PROTECTED(s) => s,
            })
            .unwrap_or("unknown".to_string());

        extract_method(&line, current_class, classes, &regex_map, &access_modifier_string);

        extract_field(line, current_class, classes, &regex_map, access_modifier_string);
    }
}

fn extract_method(line: &String, current_class: &mut String, classes: &mut HashMap<String, Class>, regex_map: &&HashMap<ClassRegex, Regex>, access_modifier_string: &String) {

    extract_captures(&regex_map[&ClassRegex::METHOD], line.as_str(), vec![2, 3, 4])
        .map(|vec| {
            let return_type = vec.get(0).unwrap();
            let method_name = vec.get(1).unwrap();

            // Collect the results into a HashMap
            let raw_parameters = vec.get(2).unwrap();
            let parameters = extract_parameters(raw_parameters);

            classes.get_mut(current_class).map(|class| {
                class.add_method(Method::new(
                    access_modifier_string.clone(),
                    method_name.to_string(),
                    return_type.to_string(),
                    parameters
                ));
            });
        });
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
                        //params.insert(_type.to_string(), _name.to_string());
                    }
                }
            }
        });

    // Return the HashMap containing all parameters
    params
}

fn extract_field(line: String, current_class: &mut String, classes: &mut HashMap<String, Class>, regex_map: &&HashMap<ClassRegex, Regex>, access_modifier_string: String) {
    // Extract attribute
    extract_captures(&regex_map[&ClassRegex::FIELD], line.as_str(), vec![1, 2])
        .map(|vec| {
            let _type = vec.get(0).unwrap();
            let _name = vec.get(1).unwrap();
            // Perform operations with the two captured strings
            classes.get_mut(current_class).map(|class| {
                class.fields().insert(Field::new(access_modifier_string.clone(), _name.to_string(), _type.to_string()));
            });
        });
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