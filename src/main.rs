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

static VERSION: f32 = 1.1;

#[tokio::main]
async fn main() -> Result<(), vrchat_osc::Error> {
    // Title
    print!("OSC Clock v{0:.1}\n", VERSION);
    print!("{}\n\n", t!("press_ctrl+c_to_exit"));

    // Load configuration
    config::init_config();

    if config::CONFIG.lock().unwrap().use_osc_query {
        osc_query::start().await?;
    } else {
        legacy::start().await;
    }

    Ok(())
}
