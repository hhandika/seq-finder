use clap::{App, AppSettings, Arg, ArgMatches};

use crate::finder;

pub fn get_cli(version: &str) {
    let args = App::new("seq-finder")
        .version(version)
        .about("Find ngs sequence")
        .author("Heru Handika <hhandi1@lsu.edu>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("find")
                .about("Find files and construct new names")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Inputs dir")
                        .takes_value(true)
                        .value_name("DIR")
                        .required(true),
                )
                .arg(
                    Arg::with_name("word")
                        .short("w")
                        .long("word")
                        .help("Numbers of words")
                        .takes_value(true)
                        .value_name("NUM WORD")
                        .required(true)
                        .default_value("3"),
                )
                .arg(
                    Arg::with_name("sep")
                        .short("s")
                        .long("sep")
                        .help("Separators")
                        .takes_value(true)
                        .value_name("SEPARATOR")
                        .required(true)
                        .default_value("_"),
                )
                .arg(
                    Arg::with_name("csv")
                        .long("csv")
                        .help("Print to csv")
                        .takes_value(false),
                ),
        )
        .get_matches();

    if let ("find", Some(clean_matches)) = args.subcommand() {
        find_files(clean_matches)
    }
}

fn find_files(matches: &ArgMatches) {
    if matches.is_present("dir") {
        let path = matches.value_of("dir").unwrap();
        let nword = get_words(matches);
        let sep = get_separators(matches);
        let iscsv = matches.is_present("csv");

        finder::find_cleaned_fastq(path, nword, sep, iscsv);
    }
}

fn get_words(matches: &ArgMatches) -> usize {
    let word = matches.value_of("word");
    match word {
        None => panic!("Unknown numbers"),
        Some(n) => n
            .parse::<usize>()
            .expect("Can't parse the input. Make sure it is an integer."),
    }
}

fn get_separators(matches: &ArgMatches) -> char {
    let sep = matches.value_of("sep");
    match sep {
        None => panic!("Unknown numbers"),
        Some(s) => s
            .parse::<char>()
            .expect("Can't parse the input. Make sure it is a single character."),
    }
}
