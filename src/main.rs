use std::{
    fs,
    io::{self, BufRead},
};

use clap::{App, Arg};
use sysinfo::{ProcessExt, System, SystemExt};

type BoxError<T> = Result<T, Box<dyn std::error::Error>>;

// Function gets the names of the current processes of the system.
fn get_cur_procs(system: &mut System) -> Vec<String> {
    system.refresh_all();

    let mut result: Vec<String> = vec![];

    system.get_processes().iter().for_each(|(_, name)| {
        result.push(String::from(name.name()));
    });

    result
}

fn parse_config(config_path: &str) -> BoxError<Vec<String>> {
    let mut result: Vec<String> = vec![];

    let file = fs::File::open(config_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let read_line = line?;
        result.push(read_line);
    }

    Ok(result)
}

fn main() -> BoxError<()> {
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
    println!("{:#?}", matches);

    // MINER and CONFIG are required, so clap ensures that it is specified, making the unwraps safe.
    let _miner_path = matches.value_of("MINER").unwrap();
    let config_path = matches.value_of("CONFIG").unwrap();

    let _game_processes = match parse_config(config_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Error parsing config file.");
            return Err("Error parsing config file.".into());
        }
    };

    let mut system = System::new_all();

    let _current_procs = get_cur_procs(&mut system);

    Ok(())
}
