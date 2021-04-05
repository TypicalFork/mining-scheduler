use clap::{App, AppSettings, Arg, SubCommand};
use sysinfo::{System, SystemExt};

mod processes;
mod schedule;
mod utils;
use processes::processes;
use schedule::schedule;

type BoxError<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main()]
async fn main() -> BoxError<()> {
    let matches = App::new("Mining Scheduler")
        .version("0.1")
        .author("Klim T. <klimusha@gmail.com>")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("schedule")
                .arg(
                    Arg::with_name("MINER")
                        .required(true)
                        .help("Sets the miner to run."),
                )
                .arg(
                    Arg::with_name("CONFIG")
                        .required(true)
                        .help("Sets the file containing a list of game process names."),
                ),
        )
        .subcommand(
            SubCommand::with_name("processes")
                .about("Displays a list of currently running processes"),
        )
        .get_matches();

    let mut system = System::new_all();

    match matches.subcommand() {
        ("schedule", Some(sub_matches)) => schedule(&mut system, sub_matches).await?,
        ("processes", Some(sub_matches)) => processes(&mut system, sub_matches).await?,
        // There will always be a subcommand as they are required.
        _ => {}
    }

    Ok(())
}
