extern crate barnacle;
extern crate clap;
extern crate config;
extern crate chrono;

use clap::{Arg, App};
use chrono::Utc;
use barnacle::data_types;
use barnacle::data_types::Validation;
use barnacle::{build_play_paths, retrieve_yaml, run_plays};
use std::collections::HashMap;

fn main() {

    let matches = App::new("Risk Barnacle")
        .long_about("A Monte Carlo simulation tool for calculating monetary losses based of different risk scenarios")
        .set_term_width(80)
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILENAME")
             .help("TODO Specify a custom output file name. Defaults to datetime.")
             .takes_value(true))
        .arg(Arg::with_name("diff")
             .short("d")
             .long("diff")
             .value_name("FILE1")
             .value_name("FILE2")
             .help("TODO See the delta in risk calculations between two output CSV files."))
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

    let mut iterations: usize = 100_000;
    if matches.is_present("iterations") {
        iterations = matches.value_of("iterations").unwrap().parse().unwrap();
    }
    let iterations = iterations;
    
    let mut output_file_name = String::from(settings.get("output_dir").expect("No value \"output_dir\" listed in Settings.toml. This is required").clone() + "/" + &Utc::now().to_rfc3339() + "UTC.csv"); 
    if matches.is_present("output") {
        output_file_name = matches.value_of("output").unwrap().to_owned()    + ".csv";
    }

    run_plays(&output_file_name, &iterations, &mut paths, &conditions, &events, &costs);
}
