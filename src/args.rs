use clap::{Arg, ArgMatches, Command};

pub fn cli_options() -> ArgMatches {
    let command = Command::new("feroxlight")
        .about("Highlight the given text with customisable regex, words and colour")
        .author("Romain BOIREAU")
        // Argument for searching in a file
        .arg(
            Arg::new("file")
                .help("Will search in the specified file")
                .value_name("FILE")
                .short('f')
                .long("file")
                .takes_value(true)
                .required(false),
        )
        // Argument if using or not default regexes
        .arg(
            Arg::new("no-default")
                .help("Do not use the default regex and color")
                .long("no-default")
                .takes_value(false)
                .required(false),
        )
        // TODO: write more information about regex, flags in the help
        // Argument to specify regex, words and their color to be used
        .args(&[
            Arg::new("color-red")
                .help("Highlight specified regex (delimited by a comma) in red")
                .value_name("REGEX1,REGEX2")
                .long("cr")
                .takes_value(true)
                .required(false),
            Arg::new("color-blue")
                .help("Highlight specified regex (delimited by a comma) in blue")
                .value_name("REGEX1,REGEX2")
                .long("cb")
                .takes_value(true)
                .required(false),
            Arg::new("color-green")
                .help("Highlight specified regex (delimited by a comma) in green")
                .value_name("REGEX1,REGEX2")
                .long("cg")
                .takes_value(true)
                .required(false),
            Arg::new("color-yellow")
                .help("Highlight specified regex (delimited by a comma) in yellow")
                .value_name("REGEX1,REGEX2")
                .long("cy")
                .takes_value(true)
                .required(false),
            Arg::new("color-magenta")
                .help("Highlight specified regex (delimited by a comma) in magenta")
                .value_name("REGEX1,REGEX2")
                .long("cm")
                .takes_value(true)
                .required(false),
            Arg::new("color-cyan")
                .help("Highlight specified regex (delimited by a comma) in cyan")
                .value_name("REGEX1,REGEX2")
                .long("cc")
                .takes_value(true)
                .required(false),
        ]);

    command.get_matches()
}
