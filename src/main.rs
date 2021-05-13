use clap::{App, AppSettings, Arg, SubCommand};
use regex::Regex;
use sysinfo::{System, SystemExt};

mod processes;
mod schedule;
mod utils;
use processes::processes;
use schedule::schedule;

type BoxError<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BoxError<()> {
    let matches = App::new("Mining Scheduler")
        .version("0.1")
        .author("Klim T. <klimusha@gmail.com>")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("schedule")
                .about("Schedule a program")
                .arg(
                    Arg::with_name("miner_path")
                        .value_name("MINER")
                        .required(true)
                        .help("Sets the miner to run"),
                )
                .arg(
                    Arg::with_name("config_path")
                        .value_name("CONFIG")
                        .required(true)
                        .help("Sets the file containing a list of game process names"),
                )
                .arg(
                    Arg::with_name("verbosity")
                        .value_name("VERBOSITY")
                        .help("Sets the verbosity level [1-3]")
                        .short("v")
                        .long("verbose")
                        .takes_value(true)
                        .validator(|arg| {
                            let re = Regex::new("[1-3]").unwrap();
                            if re.is_match(&arg) {
                                Ok(())
                            } else {
                                Err("Verbosity level must be between 1-3".into())
                            }
                        }),
                )
                .arg(
                    Arg::with_name("sleep_time")
                        .value_name("TIME")
                        .help("Sets how often the scheduler checks running processes (in seconds)")
                        .short("t")
                        .long("sleep-time")
                        .takes_value(true)
                        .validator(|arg| {
                            let re = Regex::new("[0-9]+").unwrap();
                            if re.is_match(&arg) {
                                Ok(())
                            } else {
                                Err("The sleep time can only contain digits".into())
                            }
                        }),
                )
                .arg(
                    Arg::with_name("case_insensitive")
                        .help("Sets the process names to be treated case insensitively")
                        .short("i")
                        .long("case-insensitive"),
                ),
        )
        .subcommand(
            SubCommand::with_name("processes")
                .about("Display a list of currently running processes"),
        )
        .get_matches();

    let mut system = System::new_all();

    match matches.subcommand() {
        ("schedule", Some(sub_matches)) => schedule(&mut system, sub_matches)?,
        ("processes", Some(sub_matches)) => processes(&mut system, sub_matches)?,
        // There will always be a subcommand as they are required.
        _ => {}
    }

    Ok(())
}
