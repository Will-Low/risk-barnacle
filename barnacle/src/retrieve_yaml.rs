use data_types::{Condition, Cost, Event, Play};
use std::fs;

pub fn conditions(file_name: &str) -> Vec<Condition> {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    serde_yaml::from_str(&contents).unwrap()
}

pub fn events(file_name: &str) -> Vec<Event> {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    serde_yaml::from_str(&contents).unwrap()
}

pub fn costs(file_name: &str) -> Vec<Cost> {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    serde_yaml::from_str(&contents).unwrap()
}

pub fn play(file_name: &str) -> Play {
    let contents = fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Couldn't find file '{}'", file_name));
    let mut parsed: Play = serde_yaml::from_str(&contents).unwrap();
    parsed.file_name = Some(file_name.to_string());
    parsed
}
