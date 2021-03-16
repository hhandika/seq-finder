use std::path::PathBuf;

use clap::{App, AppSettings, Arg, ArgMatches};

use crate::io;
use crate::runner;

pub fn get_cli(version: &str) {
    let args = App::new("seq-finder")
        .version(version)
        .about("Find ngs sequence")
        .author("Heru Handika <hhandi1@lsu.edu>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("find")
                .about("Runs fastp")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Inputs dir")
                        .takes_value(true)
                        .value_name("DIR")
                )
        )
        
        .get_matches();

    match args.subcommand() {
        ("find", Some(clean_matches)) => run_fastp_clean(clean_matches, version),
        ("check", Some(_)) => runner::check_fastp(),
        _ => (),
    };
}

fn run_fastp_clean(matches: &ArgMatches, version: &str) {
    if matches.is_present("input") {
        let path = PathBuf::from(matches.value_of("input").unwrap());
        let mut is_id = false;
        let mut is_rename = false;

        if matches.is_present("id") {
            is_id = true;
        }

        if matches.is_present("rename") {
            is_rename = true;
        }

        if matches.is_present("dry-run") {
            io::dry_run(&path, is_id, is_rename);
        } else {
            println!("Starting fastp-runner v{}...\n", version);
            io::process_input(&path, is_id, is_rename);
        }
    } 
}