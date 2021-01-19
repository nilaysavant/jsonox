use clap::{App, Arg};

use crate::constants::app_metadata::{APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION};

use super::tutorial::get_tutorial_text;


/// Setup CLI interface with help and args
/// Returns a matches struct from which arg values can be extracted
pub fn setup_cli_get_matches() -> clap::ArgMatches<'static> {
    // CLI Display and argument parsing
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
        .after_help(get_tutorial_text().as_str())
        .arg(
            Arg::with_name("bind_addr")
                .short("b")
                .long("bind-addr")
                .value_name("IP:PORT")
                .help("Set bind address (defaults to 0.0.0.0:8080)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Quiet mode with no logging"),
        )
        .get_matches()
}
