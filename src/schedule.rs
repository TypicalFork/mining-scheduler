use std::{thread, time};

use clap::ArgMatches;
use sysinfo::System;
use tokio::process;

use crate::utils;
use crate::BoxError;

pub async fn schedule(system: &mut System, matches: &ArgMatches<'_>) -> BoxError<()> {
    // MINER and CONFIG are required, so clap ensures that it is specified, making the unwraps safe.
    let miner_path = matches.value_of("MINER").unwrap();
    let config_path = matches.value_of("CONFIG").unwrap();

    let game_procs = match utils::parse_config(config_path) {
        Ok(c) => c,
        Err(_) => {
            return Err("Error parsing config file.".into());
        }
    };

    let mut miner_proc: Option<tokio::process::Child> = None;

    let mut game_running_prev_iter = true;

    loop {
        let current_procs = utils::get_cur_procs(system);

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
