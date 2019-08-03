use data_types::{Condition, Cost, Event, Play};
use std::fs;
use std::process;

pub fn conditions(file_name: &str) -> Vec<Condition> {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    serde_yaml::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Something when wrong when trying to parse conditions.yaml. \
                  Please review for errors.");
        process::exit(1);
    })
}

pub fn events(file_name: &str) -> Vec<Event> {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| {
            eprintln!("Couldn't find file '{}'", file_name);
            process::exit(1);
        });
    serde_yaml::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Something when wrong when trying to parse events.yaml. \
                  Please review for errors.");
        process::exit(1);
    })
}

pub fn costs(file_name: &str) -> Vec<Cost> {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    serde_yaml::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Something when wrong when trying to parse costs.yaml. \
                  Please review for errors.");
        process::exit(1);
    })
}

pub fn play(file_name: &str) -> Play {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    let mut parsed: Play = serde_yaml::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Something when wrong when trying to parse \"{}\". \
                  Please review for errors.", &file_name);
        process::exit(1);
    });
    parsed.file_name = Some(file_name.to_string());
    parsed
}
