use clap::{App, Arg, SubCommand};
use sysinfo::{System, SystemExt};

mod schedule;
use schedule::schedule;

type BoxError<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main()]
async fn main() -> BoxError<()> {
    let matches = App::new("Mining Scheduler")
        .version("0.1")
        .author("Klim T. <klimusha@gmail.com>")
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
        .get_matches();

    let mut system = System::new_all();

    schedule(&mut system, matches).await?;
    Ok(())
}
