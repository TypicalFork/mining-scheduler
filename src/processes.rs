use clap::ArgMatches;
use sysinfo::System;

use crate::utils;
use crate::BoxError;

pub fn processes(system: &mut System, matches: &ArgMatches<'_>) -> BoxError<()> {
    let case_insensitive = matches.is_present("case_insensitive");
    utils::get_current_processes(system, case_insensitive)
        .iter()
        .for_each(|proc| println!("{}", proc));
    Ok(())
}
