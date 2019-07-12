pub mod calculation;
pub mod data_types;
pub mod retrieve_yaml;

extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate walkdir;

use data_types::Validation;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn run_plays(
    iterations: &usize,
    paths: &mut Vec<String>,
    conditions: &[data_types::Condition],
    events: &[data_types::Event],
    costs: &[data_types::Cost],
) {
    let mut total = data_types::MonteCarloResult {
        description: String::from("TOTAL"),
        annual_loss_event_prob: 0.0,
        fifth_percentile: 0.0,
        ninety_fifth_percentile: 0.0,
        mean: 0.0,
        median: 0.0,
        std_dev: 0.0,
    };
    for path in paths {
        let mut play: data_types::Play = retrieve_yaml::play(&path);
        play.validate();
        play.build_models(&events, &conditions, &costs);
        let result = calculation::monte_carlo(&iterations, &play);

        println!(
            "{0: <60} | {1:9.6}% | {2:10.2} | {3:10.2} | {4:10.2} | {5:10.2} | {6:10.2}",
            result.description,
            result.annual_loss_event_prob,
            result.fifth_percentile,
            result.ninety_fifth_percentile,
            result.mean,
            result.median,
            result.std_dev
        );

        total.fifth_percentile += result.fifth_percentile;
        total.ninety_fifth_percentile += result.ninety_fifth_percentile;
        total.mean += result.mean;
        total.median += result.median;
        total.std_dev += result.std_dev;
    }

    println!(
        "{0: <60} | {1:9.6}% | {2:10.2} | {3:10.2} | {4:10.2} | {5:10.2} | {6:10.2}",
        total.description,
        total.annual_loss_event_prob,
        total.fifth_percentile,
        total.ninety_fifth_percentile,
        total.mean,
        total.median,
        total.std_dev
    );
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn build_play_paths(play_dir: &str) -> Vec<String> {
    let play_path = Path::new(play_dir);
    let mut paths: Vec<String> = vec![];

    // Skip hidden files
    let walker = WalkDir::new(play_path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok());

    for entry in walker {
        if entry.file_type().is_file() {
            paths.push(entry.path().to_str().unwrap().to_string());
        }
    }
    paths
}
