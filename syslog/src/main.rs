fn main() {
    println!("Hello, world!");
}
#[macro_use]
extern crate log;
extern crate syslog;

use log::LogLevelFilter;
use syslog::Facility;

fn run() -> Result<()> {
    syslog::init(Facility::LOG_USER,
                 LogLevelFilter::Debug,
                 Some("My app name"))?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}
