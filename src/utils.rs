use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
};

use sysinfo::{ProcessExt, System, SystemExt};

use crate::BoxError;

pub fn get_current_processes(system: &mut System, case_insensitive: bool) -> HashSet<String> {
    system.refresh_all();

    let mut result: HashSet<String> = HashSet::new();

    system.get_processes().iter().for_each(|(_, name)| {
        let proc = String::from(name.name());
        if case_insensitive {
            result.insert(proc.to_lowercase());
        } else {
            result.insert(proc);
        }
    });

    result
}

pub fn parse_config(config_path: &str, case_insensitive: bool) -> BoxError<HashSet<String>> {
    let mut result: HashSet<String> = HashSet::new();

    let file = fs::File::open(config_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let game = line?;
        if case_insensitive {
            result.insert(game.to_lowercase());
        } else {
            result.insert(game);
        }
    }

    Ok(result)
}
