use std::io::Error;

mod args;
mod config;
mod ping;
mod ping_entry;

fn main() -> Result<(), Error> {
    ping::run()
}
