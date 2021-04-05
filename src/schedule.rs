use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
    thread, time,
};

use clap::ArgMatches;
use sysinfo::{ProcessExt, System, SystemExt};
use tokio::process;

use crate::BoxError;

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

pub async fn schedule(system: &mut System, matches: ArgMatches<'_>) -> BoxError<()> {
    // MINER and CONFIG are required, so clap ensures that it is specified, making the unwraps safe.
    let miner_path = matches.value_of("MINER").unwrap();
    let config_path = matches.value_of("CONFIG").unwrap();

    let game_procs = match parse_config(config_path) {
        Ok(c) => c,
        Err(_) => {
            return Err("Error parsing config file.".into());
        }
    };

    let mut miner_proc: Option<tokio::process::Child> = None;

    let mut game_running_prev_iter = true;

    loop {
        let current_procs = get_cur_procs(system);

        let game_running_curr_iter = current_procs.intersection(&game_procs).count() > 0;

        if game_running_curr_iter != game_running_prev_iter {
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
            };
            game_running_prev_iter = game_running_curr_iter;
        }

        thread::sleep(time::Duration::from_secs(10));
    }
}
