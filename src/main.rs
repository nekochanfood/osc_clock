extern crate rosc;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

mod config;
use config::{ init_config, CONFIG };

mod log;

mod recovery;

mod sender;
mod receiver;

mod message;
mod legacy;

use vrchat_osc::{ Error };

mod osc_query;

static VERSION: f32 = 1.0;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Title
    print!("OSC Clock v{0:.1}\n", VERSION);
    print!("{}\n\n", t!("press_ctrl+c_to_exit"));

    // Load configuration
    init_config();

    if CONFIG.lock().unwrap().use_osc_clock {
        osc_query::start().await?;
    } else {
        legacy::start().await;
    }

    Ok(())
}
