extern crate rosc;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod config;
mod legacy;
mod log;
mod message;
mod osc_query;
mod receiver;
mod recovery;
mod sender;
mod order;
mod unit;
mod arg;

#[tokio::main]
async fn main() -> Result<(), vrchat_osc::Error> {
    // Check arguments
    arg::check_args();

    // Display Title
    print!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    print!("{}\n\n", t!("press_ctrl+c_to_exit"));

    // Init and load configuration
    config::init_config();

    // Init and load orders
    order::init_orders();

    // Choose the communication method based on the configuration
    if config::CONFIG.lock().unwrap().use_osc_query {
        // Start with OSC Query
        osc_query::start().await?;
    } else {
        // Start with normal OSC communication
        legacy::start().await;
    }

    Ok(())
}
