use clap::ArgMatches;
use sysinfo::System;

use crate::utils;
use crate::BoxError;

pub async fn processes(system: &mut System, matches: &ArgMatches<'_>) -> BoxError<()> {
    let case_insensitive = match matches.value_of("case_insensitive") {
        Some(_) => true,
        None => false,
    };
    utils::get_current_processes(system, &case_insensitive)
        .into_iter()
        .for_each(|proc| println!("{}", proc));
    Ok(())
}
