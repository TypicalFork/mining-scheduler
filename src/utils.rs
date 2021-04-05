use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
};

use sysinfo::{ProcessExt, System, SystemExt};

use crate::BoxError;

pub fn get_cur_procs(system: &mut System) -> HashSet<String> {
    system.refresh_all();

    let mut result: HashSet<String> = HashSet::new();

    system.get_processes().iter().for_each(|(_, name)| {
        result.insert(String::from(name.name()));
    });

    result
}

pub fn parse_config(config_path: &str) -> BoxError<HashSet<String>> {
    let mut result: HashSet<String> = HashSet::new();

    let file = fs::File::open(config_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let read_line = line?;
        result.insert(read_line);
    }

    Ok(result)
}
