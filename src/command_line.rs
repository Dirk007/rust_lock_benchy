use clap::{App, Arg, ArgMatches};

/// Just parse our command line and return the matches
pub fn parse_arguments<'a>() -> ArgMatches<'a> {
    App::new("Mux benchy")
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("print version")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("reads")
                .short("r")
                .long("reads")
                .help("How many reads per run")
                .default_value("40000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("writemod")
                .short("w")
                .long("writemod")
                .help("Modulo for writes against reads")
                .default_value("10000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tasks")
                .short("t")
                .long("tasks")
                .help("Tasks to run")
                .default_value("100")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("writers")
                .short("p")
                .long("writers")
                .help("Tasks to act as writers (rest = readers)")
                .default_value("2")
                .takes_value(true),
        )
        .get_matches()
}
