mod args;
mod config;
mod ping;
mod ping_entry;

fn main() {
    match ping::run() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
