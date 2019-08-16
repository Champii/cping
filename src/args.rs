use clap::{App, Arg};

use super::config::Config;

pub fn parse_config() -> Config {
    let matches = App::new("CPing")
        .version("0.1")
        .author("Champii <contact@champii.io>")
        .about("Colorful Ping")
        .arg(
            Arg::with_name("no-legend")
                .short("L")
                .long("no-legend")
                .help("Hide the legend"),
        )
        .arg(
            Arg::with_name("no-history")
                .short("H")
                .long("no-history")
                .help("Hide the history"),
        )
        .arg(
            Arg::with_name("no-graph")
                .short("G")
                .long("no-graph")
                .help("Hide the graph"),
        )
        .arg(
            Arg::with_name("no-title")
                .short("T")
                .long("no-title")
                .help("Hide the title"),
        )
        .arg(
            Arg::with_name("address")
                .help("the address/domain to ping")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("ping-args")
                .help("Pass arguments to ping")
                .last(true),
        )
        .get_matches();

    let no_legend = matches.is_present("no-legend");
    let no_history = matches.is_present("no-history");
    let no_graph = matches.is_present("no-graph");
    let no_title = matches.is_present("no-title");

    let addr = matches.value_of("address").unwrap();
    let mut ping_args: Vec<String> = matches
        .values_of("ping-args")
        .map(|items| {
            items
                .collect::<Vec<_>>()
                .iter()
                .map(|item| item.to_string())
                .collect()
        })
        .unwrap_or_default();

    ping_args.push(addr.to_string());

    Config {
        no_legend,
        no_history,
        no_graph,
        no_title,
        addr: addr.to_string(),
        ping_args,
    }
}
