use clap::{Arg, Command};
use crate::recovery::repair;

pub fn check_args() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("repair")
                .short('r')
                .long("repair")
                .help("Repair config file")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    if matches.get_flag("repair") {
        repair();
    }
}

