use clap::ArgMatches;
use sysinfo::System;

use crate::utils;
use crate::BoxError;

pub async fn processes(system: &mut System, matches: &ArgMatches<'_>) -> BoxError<()> {
    utils::get_cur_procs(system)
        .into_iter()
        .for_each(|proc| println!("{}", proc));
    Ok(())
}
