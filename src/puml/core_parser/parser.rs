use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use crate::puml::code_generators::code_generator::{SourceCodeGenerator, SourceCodeStrategy};
use crate::puml::common::constants::{CH_PRIVATE, CH_PROTECTED, CH_PUBLIC};

use crate::puml::core_parser::class::{AccessModifier, Attribute, Class, Method};
use crate::puml::core_parser::regex::{REGEX_SINGLETON, RegexType};

pub fn parse(path: String, source_code_strategy: SourceCodeStrategy) -> HashMap<String, String> {

    let start = Instant::now();

    // Trim the input path to remove any extra spaces, newline, or carriage return characters
    let path = path.trim();
    let parsing_res = parse_puml(path);

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

fn parse_puml(path: &str) -> Result<HashMap<String, Class>, String> {

    let start = Instant::now();

    // Attempt to open the file
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            let mut current_class = String::new();
            let mut classes = HashMap::new();

            let start_for = Instant::now();
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
            println!("parse_puml() - for cycle time: {:?}", start_for.elapsed());
            let method_duration = start.elapsed();
            println!("parse_puml() time: {:?}", method_duration);
            // Return the classes if everything went well
            Ok(classes)
        }
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }
}

fn process_line(line: String, current_class: &mut String, classes: &mut HashMap<String, Class>) {

    // let regex_mutex = REGEX_SINGLETON.lock().unwrap();

    let regex_map = REGEX_SINGLETON.regexes();

    // Detect class definition
    regex_map.get(&RegexType::CLASS).expect("Regex not found").captures(line.as_str())
        .and_then(|cap| cap.get(1))
        .map(|matched_string| {
            current_class.clear();
            current_class.push_str(matched_string.as_str());
            classes.insert(current_class.clone(), Class::new());
        });

    // Detect parent class
    regex_map.get(&RegexType::PARENT).expect("Regex not found").captures(line.as_str())
        .and_then(|cap| cap.get(1))
        .map(|matched_string| {
            if let Some(class) = classes.get_mut(current_class) {
                class.extended_class().to_string().push_str(matched_string.as_str());
            }
        });

    // Detect interface
    regex_map.get(&RegexType::INTERFACE).expect("Regex not found").captures(line.as_str())
        .and_then(|cap| cap.get(1))
        .map(|matched_string| {
            if let Some(class) = classes.get_mut(current_class) {
                class.interface().to_string().push_str(matched_string.as_str());
            }
        });

    // process members
    if !current_class.is_empty() && (line.contains(CH_PRIVATE) || line.contains(CH_PUBLIC) || line.contains(CH_PROTECTED)) {

        // Detect access modifier
        let access_modifier_string = line.chars().next()
            .and_then(|first_char| get_access_modifier(first_char))  // Get the access modifier
            .map(|modifier| match modifier {
                AccessModifier::PUBLIC(s) => s,
                AccessModifier::PRIVATE(s) => s,
                AccessModifier::PROTECTED(s) => s,
            }) // Map enum to the associated string
            .unwrap_or("unknown".to_string());  // Default to "unknown" if no match

        // Extract method
        regex_map.get(&RegexType::METHOD).expect("Regex not found").captures(line.as_str())
            .and_then(|cap| {
                // group 1: access modifier; group 2: return type; group 3: method name
                Some((cap.get(2)?.as_str(), cap.get(3)?.as_str()))
            })
            .map(|(return_type, method_name)| {
                // Perform operations with the two captured strings
                classes.get_mut(current_class).
                    map(|class| {
                        class.methods().push(Method::new(access_modifier_string.clone(), method_name.to_string(), return_type.to_string()));
                    })
            });

        // Extract attribute
        regex_map.get(&RegexType::ATTRIBUTE).expect("Regex not found").captures(line.as_str())
            .and_then(|cap| {
                // group 1: type; group 2: name;
                Some((cap.get(1)?.as_str(), cap.get(2)?.as_str()))
            })
            .map(|(_type, _name)| {
                // Perform operations with the two captured strings
                classes.get_mut(current_class).
                    map(|class| {
                        class.attributes().push(Attribute::new(access_modifier_string.clone(), _name.to_string(), _type.to_string()));
                    })
            });
    }
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