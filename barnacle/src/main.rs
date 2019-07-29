extern crate barnacle;
extern crate chrono;
extern crate clap;
extern crate config;

use barnacle::data_types;
use barnacle::data_types::Validation;
use barnacle::{build_play_paths, retrieve_yaml, run_plays};
use chrono::Utc;
use clap::{App, Arg};
use std::collections::HashMap;

fn main() {
    let matches = App::new("Risk Barnacle")
        .name("RISK BARNACLE")
        .long_about("A Monte Carlo simulation tool for calculating monetary losses based of different risk scenarios")
        .version("v. 0.0.1")
        .set_term_width(80)
        .after_help("All dependent file locations and directories are located in the Settings.toml file.")
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILENAME")
             .help("Specify a custom output file name. Defaults to datetime.")
             .takes_value(true))
        .arg(Arg::with_name("iterations")
             .short("i")
             .long("iterations")
             .value_name("ITERATIONS")
             .help("Specify a custom iteration count for the Monte Carlo simulation. Default is 100,000."))
        .get_matches();

    // Load config
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let settings = settings.try_into::<HashMap<String, String>>().unwrap();

    // Gather reference values
    let conditions: Vec<data_types::Condition> =
        retrieve_yaml::conditions(settings.get("conditions").unwrap());
    let events: Vec<data_types::Event> = retrieve_yaml::events(settings.get("events").unwrap());
    let costs: Vec<data_types::Cost> = retrieve_yaml::costs(settings.get("costs").unwrap());

    conditions.validate();
    events.validate();
    costs.validate();

    // Build list of paths to plays
    let mut paths = build_play_paths(settings.get("plays").unwrap());

    let iterations: usize = if matches.is_present("iterations") {
        matches.value_of("iterations").unwrap().parse().unwrap()
    } else {
        100_000
    };

    let output_file_name = if matches.is_present("output") {
        matches.value_of("output").unwrap().to_owned() + ".csv"
    } else {
        settings
            .get("output_dir")
            .expect("No value \"output_dir\" listed in Settings.toml. This is required")
            .clone()
            + "/"
            + &Utc::now().to_rfc3339()
            + "UTC.csv"
    };

    run_plays(
        &output_file_name,
        iterations,
        &mut paths,
        &conditions,
        &events,
        &costs,
    );
}
