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
use std::process;

fn main() {
    let cli_app = build_cli();
    let matches = cli_app.get_matches();

    // Load config
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap_or_else(|_| {
        eprintln!("Unable to find the config file \"Settings.toml\" in the \
                  current directory. This file is required.");
        process::exit(1);
    });
    let settings = settings.try_into::<HashMap<String, String>>().unwrap_or_else(|_| {
        eprintln!("Unable to understand \"Settings.toml\". Please review the \
                  formatting for errors."); 
        process::exit(1);
    });

    // Gather reference values
    let conditions: Vec<data_types::Condition> =
        retrieve_yaml::conditions(settings.get("conditions").unwrap_or_else(|| {
            eprintln!("No location for the \"conditions\" file specified in \
                      \"Settings.toml\". This value is required.");
            process::exit(1);
        
        }));
    let events: Vec<data_types::Event> = 
        retrieve_yaml::events(settings.get("events").unwrap_or_else(||{
            eprintln!("No location for the \"events\" file specified in \
                      \"Settings.toml\". This value is required.");
            process::exit(1);
        }));
    let costs: Vec<data_types::Cost> = 
        retrieve_yaml::costs(settings.get("costs").unwrap_or_else(|| {
            eprintln!("No location for the \"costs\" file specified in \
                      \"Settings.toml\". This value is required.");
            process::exit(1);
        }));

    conditions.validate();
    events.validate();
    costs.validate();

    // Build list of paths to plays
    let mut paths = build_play_paths(settings.get("plays").unwrap_or_else(|| {
            eprintln!("No location for the \"plays\" directory specified in \
                      \"Settings.toml\". This value is required.");
            process::exit(1);
    }));

    let iterations: usize = if matches.is_present("iterations") {
        matches.value_of("iterations").unwrap().parse().unwrap()
    } else {
        100_000
    };

    let output_file_name = if matches.is_present("output") {
        "output/".to_owned() + matches.value_of("output").unwrap() + ".csv"
    } else {
        settings
            .get("output_dir")
            .unwrap_or_else(|| {
                eprintln!("No value \"output_dir\" listed in Settings.toml. \
                          This is required");
                process::exit(1);
            })
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

fn build_cli<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("Risk Barnacle")
            .name("RISK BARNACLE")
            .about("A Monte Carlo simulation tool for calculating monetary \
                        losses based of different risk scenarios")
            .version("v. 0.1.0")
            .set_term_width(80)
            .after_help("All dependent file locations and directories are located \
                        in the Settings.toml file.")
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
                 .help("Specify a custom iteration count for the Monte Carlo \
                       simulation. Default is 100,000."))   
}
