extern crate barnacle;
extern crate clap;
extern crate config;

use clap::{Arg, App};
use barnacle::data_types;
use barnacle::data_types::Validation;
use barnacle::{build_play_paths, retrieve_yaml, run_plays};
use std::collections::HashMap;

fn main() {

    let matches = App::new("Phagocyte")
        .long_about("A Monte Carlo simulation tool for calculating monetary losses based of different risk scenarios")
        .author("Will Springer")
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILENAME")
             .help("Specify a custom output file name. Defaults to datetime.")
             .takes_value(true))
        .arg(Arg::with_name("diff")
             .short("d")
             .long("diff")
             .value_name("FILE1")
             .value_name("FILE2")
             .help("See the delta in risk calculations between two output CSV files."))
        .arg(Arg::with_name("iterations")
             .short("i")
             .long("iterations")
             .value_name("ITERATIONS")
             .long_help("Specify a custom iteration count for the Monte Carlo simulation. Default is 100,000."))
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

    // Table headers
    println!(
        "{0: <60} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10}",
        "PLAY-DESCRIPTION", "ANNU. PROB", "5%", "95%", "MEAN", "MEDIAN", "STD DEV"
    );

    let iterations = 100_000;
    run_plays(&mut paths, &conditions, &events, &costs);
}
