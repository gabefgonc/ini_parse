use std::collections::HashMap;
use std::fs::read_to_string;


fn main() {
    let parsed = parse("test.ini");
    for (section, values) in parsed {
        println!("Section: {section}");
        for (key, value) in values {
            println!("{key} = {value}")
        }
    }
}

fn parse(filename: &str) -> HashMap<String, HashMap<String, String>> {
    let file = read_to_string(filename).unwrap();
    let lines = file.lines().map(String::from);
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();

    let mut current_section_name: String = String::from("");
    for (idx, line) in lines.enumerate() {
        if line.trim() == "" {
            continue;
        }
        if line.starts_with("[") && line.ends_with("]") {
            current_section_name = line.replace("[", "").replace("]", "");
            sections.insert(String::clone(&current_section_name), HashMap::new());
            continue;
        }
        if let Some((key, value)) = line.trim().split_once("=") {
            sections
                .get_mut(&mut current_section_name)
                .unwrap()
                .insert(key.to_string(), value.to_string());
        } else {
            panic!("syntax error on line {idx}")
        }
    }
    sections
}
