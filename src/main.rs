use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
    thread, time,
};

use tokio::process;

use clap::{App, Arg};
use sysinfo::{ProcessExt, System, SystemExt};

type BoxError<T> = Result<T, Box<dyn std::error::Error>>;

// Function gets the names of the current processes of the system.
fn get_cur_procs(system: &mut System) -> HashSet<String> {
    system.refresh_all();

    let mut result: HashSet<String> = HashSet::new();

    system.get_processes().iter().for_each(|(_, name)| {
        result.insert(String::from(name.name()));
    });

    result
}

fn parse_config(config_path: &str) -> BoxError<HashSet<String>> {
    let mut result: HashSet<String> = HashSet::new();

    let file = fs::File::open(config_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let read_line = line?;
        result.insert(read_line);
    }

    Ok(result)
}

#[tokio::main()]
async fn main() -> BoxError<()> {
    let matches = App::new("Mining Scheduler")
        .version("0.1")
        .author("Klim T. <klimusha@gmail.com>")
        .arg(
            Arg::with_name("MINER")
                .required(true)
                .help("Sets the miner to run."),
        )
        .arg(
            Arg::with_name("CONFIG")
                .required(true)
                .help("Sets the file containing a list of game process names."),
        )
        .get_matches();

    // MINER and CONFIG are required, so clap ensures that it is specified, making the unwraps safe.
    let miner_path = matches.value_of("MINER").unwrap();
    let config_path = matches.value_of("CONFIG").unwrap();

    let game_procs = match parse_config(config_path) {
        Ok(c) => c,
        Err(_) => {
            return Err("Error parsing config file.".into());
        }
    };

    let mut system = System::new_all();

    let mut miner_proc: Option<tokio::process::Child> = None;

    loop {
        let current_procs = get_cur_procs(&mut system);

        if current_procs.intersection(&game_procs).count() > 0 {
            miner_proc = match miner_proc {
                Some(mut proc) => {
                    println!("Killing miner");
                    proc.kill().await?;
                    None
                }
                None => {
                    println!("Launching miner");
                    Some(process::Command::new(miner_path).spawn()?)
                }
            }
        }

        thread::sleep(time::Duration::from_secs(30));
    }
}
