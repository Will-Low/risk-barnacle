pub mod calculation;
pub mod data_types;
pub mod retrieve_yaml;

extern crate chrono;
#[macro_use] extern crate prettytable;
extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate walkdir;

use chrono::Utc;
use data_types::Validation;
use prettytable::Table;
use std::fs::File;
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
    let mut table = Table::new();
    let out_file = File::create(String::from("output/") + &Utc::now().to_rfc3339() + "UTC.csv").unwrap();
    table.add_row(row!["PLAY-DESCRIPTION", 
                       "ANN. PROB",
                       "5%",
                       "95%",
                       "MEAN",
                       "MEDIAN",
                       "STD DEV"]);

    for path in paths {
        let mut play: data_types::Play = retrieve_yaml::play(&path);
        play.validate();
        play.build_models(&events, &conditions, &costs);
        let result = calculation::monte_carlo(&iterations, &play);
        
        table.add_row(row![&result.description,
                           format!("{}{}", &result.annual_loss_event_prob, "%"),
                           format!("{:.2}", &result.fifth_percentile),
                           format!("{:.2}", &result.ninety_fifth_percentile),
                           format!("{:.2}", &result.mean),
                           format!("{:.2}", &result.median),
                           format!("{:.2}", &result.std_dev)]);

        total.fifth_percentile += result.fifth_percentile;
        total.ninety_fifth_percentile += result.ninety_fifth_percentile;
        total.mean += result.mean;
        total.median += result.median;
        total.std_dev += result.std_dev;
    }
     
    table.add_row(row![&total.description,
                       format!("{}{}", &total.annual_loss_event_prob, "%"),
                       format!("{:.2}", &total.fifth_percentile),
                       format!("{:.2}", &total.ninety_fifth_percentile),
                       format!("{:.2}", &total.mean),
                       format!("{:.2}", &total.median),
                       format!("{:.2}", &total.std_dev)]);
    table.printstd();
    table.to_csv(out_file).unwrap();
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
