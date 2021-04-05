use std::{thread, time};

use clap::ArgMatches;
use sysinfo::System;
use tokio::process;

use crate::utils;
use crate::BoxError;

pub async fn schedule(system: &mut System, matches: &ArgMatches<'_>) -> BoxError<()> {
    // MINER and CONFIG are required, so clap ensures that it is specified.
    let miner_path = matches.value_of("miner_path").unwrap();
    let config_path = matches.value_of("config_path").unwrap();
    let verbose: u8 = match matches.value_of("verbose") {
        // The argument is guaranteed to be between 0 - 3 due to a regex check earlier.
        Some(v) => v.parse().unwrap(),
        None => 1,
    };
    let case_insensitive = matches.is_present("case_insensitive");

    let game_procs = match utils::parse_config(config_path, &case_insensitive) {
        Ok(c) => c,
        Err(_) => {
            return Err("Error parsing config file.".into());
        }
    };

    let mut miner_proc: Option<tokio::process::Child> = None;

    let mut game_running_prev_iter = true;

    loop {
        if verbose >= 2 {
            println!("Checking running processes.");
        }

        let curr_procs = utils::get_current_processes(system, &case_insensitive);

        if verbose >= 3 {
            curr_procs
                .into_iter()
                .for_each(|proc| println!("{}", proc));
        }

        let game_running_curr_iter = curr_procs.intersection(&game_procs).count() > 0;

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
