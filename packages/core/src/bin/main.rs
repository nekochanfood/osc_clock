use clap::{Arg, Command};

#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("locales");

fn check_args() {
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
        osc_clock::recovery::repair();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check arguments (--repair flag, etc.)
    check_args();

    // Display Title
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}\n", t!("press_ctrl+c_to_exit"));

    // Run the service
    osc_clock::run().await?;

    Ok(())
}
