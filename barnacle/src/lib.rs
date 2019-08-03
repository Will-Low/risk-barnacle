pub mod calculation;
pub mod data_types;
pub mod retrieve_yaml;

extern crate chrono;
#[macro_use]
extern crate prettytable;
extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate walkdir;

use data_types::MonteCarloResult;
use data_types::Validation;
use prettytable::Table;
use std::fs::File;
use std::path::Path;
use std::process;
use walkdir::{DirEntry, WalkDir};

pub fn run_plays(
    output_file_name: &str,
    iterations: usize,
    paths: &mut Vec<String>,
    conditions: &[data_types::Condition],
    events: &[data_types::Event],
    costs: &[data_types::Cost],
) {
    let mut total = data_types::MonteCarloResult {
        description: String::from("PER ANNUM"),
        annual_loss_event_prob: 0.0,
        fifth_percentile: 0.0,
        ninety_fifth_percentile: 0.0,
        mean: 0.0,
        median: 0.0,
        std_dev: 0.0,
    };
    let mut table = Table::new();
    table.add_row(row![
        "PLAY-DESCRIPTION",
        "ANN. LOSS PROB",
        "LOW",
        "HIGH",
        "MEAN",
        "MEDIAN",
        "STD DEV"
    ]);

    let mut results: Vec<MonteCarloResult> = vec![];
    let mut results_scenarios_preserved: Vec<Vec<f64>> = vec![];

    for path in paths {
        let mut play: data_types::Play = retrieve_yaml::play(&path);
        play.validate();
        play.build_models(&events, &conditions, &costs);
        let result = calculation::monte_carlo(iterations, &play);
        results.push(result.0);
        results_scenarios_preserved.push(result.1);
    }

    if results.is_empty() {
        eprintln!("Appears there are no plays in scope.");
        process::exit(1);
    } else if results.len() == 1 {
        total.annual_loss_event_prob = results[0].annual_loss_event_prob;
        total.fifth_percentile = results[0].fifth_percentile;
        total.ninety_fifth_percentile = results[0].ninety_fifth_percentile;
        total.mean = results[0].mean;
        total.median = results[0].median;
        total.std_dev = results[0].std_dev;
        
        table.add_row(row![
            &results[0].description,
            format!("{}{}", &results[0].annual_loss_event_prob, "%"),
            format!("{:.2}", &results[0].fifth_percentile),
            format!("{:.2}", &results[0].ninety_fifth_percentile),
            format!("{:.2}", &results[0].mean),
            format!("{:.2}", &results[0].median),
            format!("{:.2}", &results[0].std_dev)
        ]);
    } else {
        for result in &results {
            table.add_row(row![
                &result.description,
                format!("{}{}", &result.annual_loss_event_prob, "%"),
                format!("{:.2}", &result.fifth_percentile),
                format!("{:.2}", &result.ninety_fifth_percentile),
                format!("{:.2}", &result.mean),
                format!("{:.2}", &result.median),
                format!("{:.2}", &result.std_dev)
            ]);
        }

        // Calculating the "per annum" fields
        let mut scenario_totals: Vec<f64> = vec![];
        for scenario_index in 0..iterations - 1 {
            let mut scenario_total = 0.0;
            for play in &results_scenarios_preserved {
                scenario_total += play[scenario_index];
            }
            if scenario_total > 0.0 {
                scenario_totals.push(scenario_total)
            }
        }
        scenario_totals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let scenario_totals_length = scenario_totals.len();

        total.annual_loss_event_prob = scenario_totals_length as f64 / iterations as f64 * 100.0;
        total.fifth_percentile = scenario_totals[0];
        total.ninety_fifth_percentile = scenario_totals[scenario_totals_length - 1];
        total.median = calculation::median(&scenario_totals).unwrap();
        total.mean = calculation::mean(&scenario_totals).unwrap();
        total.std_dev = calculation::std_deviation(&scenario_totals).unwrap();
    }

    table.add_row(row![
        &total.description,
        format!("{}{}", &total.annual_loss_event_prob, "%"),
        format!("{:.2}", &total.fifth_percentile),
        format!("{:.2}", &total.ninety_fifth_percentile),
        format!("{:.2}", &total.mean),
        format!("{:.2}", &total.median),
        format!("{:.2}", &total.std_dev)
    ]);
    table.printstd();

    let out_file = File::create(output_file_name).unwrap();
    table.to_csv(out_file).unwrap();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
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
