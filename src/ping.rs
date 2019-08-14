use std::io::{self, Write};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor, style};

use super::config::Config;
use super::ping_entry::PingEntry;

pub struct Ping {
    pub config: Config,
    pub header: String,
    pub histo: Vec<PingEntry>,
    pub term_size: (u16, u16),
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Ping {
    fn new(config: Config) -> Ping {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let term_size = termion::terminal_size().unwrap();

        Ping {
            config,
            header: String::new(),
            histo: vec![],
            stdout,
            term_size,
        }
    }

    fn add(&mut self, entry: PingEntry) {
        self.histo.push(entry);
    }

    fn print_legend(&self, pos: (u16, u16)) {
        let times = vec![10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0];

        print!(
            "{}{}Legend: ",
            cursor::Goto(1, pos.1),
            color::Fg(color::White)
        );

        for time in times {
            print!("{} < {}ms ", PingEntry::get_histo_char(time), time);
        }

        print!("{} > {}ms", PingEntry::get_histo_char(1000.1), 1000.0);
    }

    fn print_history(&self, pos: (u16, u16)) {
        let mut i = 0;

        let legend_offset = if self.config.no_legend { 0 } else { 1 };
        let graph_offset = if self.config.no_graph { 0 } else { 1 };
        let title_offset = if self.config.no_title { 0 } else { 1 };

        let low_bound = self.term_size.1 as usize - legend_offset - graph_offset - title_offset;

        let histos = if self.histo.len() > low_bound {
            self.histo.as_slice()[self.histo.len() - (low_bound)..self.histo.len()].to_vec()
        } else {
            self.histo.clone()
        };

        for histo in &histos {
            print!("{}", cursor::Goto(1, pos.1 + i));

            histo.print();
            i += 1;
        }
    }
    fn print_histogram(&self, pos: (u16, u16)) {
        print!("{}", cursor::Goto(1, pos.1));

        let histos = if self.histo.len() > self.term_size.0 as usize {
            self.histo.as_slice()[self.histo.len() - self.term_size.0 as usize..self.histo.len()]
                .to_vec()
        } else {
            self.histo.clone()
        };

        for histo in &histos {
            print!("{}", PingEntry::get_histo_char(histo.time))
        }
    }
    fn print(&mut self) {
        self.term_size = termion::terminal_size().unwrap();

        if self.term_size.0 == 0 || self.term_size.1 == 0 {
            println!("Error: Too small terminal");
            return;
        }

        print!("{}", termion::clear::All);

        let history_pos = if !self.config.no_title {
            print!(
                "{}{}PING: {}",
                cursor::Goto(1, 1),
                style::Reset,
                self.config.addr,
            );
            (1, 2)
        } else {
            (1, 1)
        };

        let graph_pos = if !self.config.no_history {
            self.print_history(history_pos);

            if self.config.no_legend {
                (1, self.term_size.1)
            } else {
                (1, self.term_size.1 - 1)
            }
        } else {
            history_pos
        };

        let legend_pos = if !self.config.no_graph {
            self.print_histogram(graph_pos);
            (1, graph_pos.1 + 1)
        } else {
            graph_pos
        };

        if !self.config.no_legend {
            self.print_legend(legend_pos);
        }
    }
}

pub fn run() -> Result<(), Error> {
    let config = super::args::parse_config();

    let mut stdout = io::stdout().into_raw_mode().unwrap();

    print!("{}", cursor::Hide);

    std::thread::spawn(move || {
        let mut ctx = Ping::new(config.clone());

        let stdout = Command::new("ping")
            .args(&[&config.addr])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
            .unwrap();

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| line.find("ttl").is_some())
            .map(PingEntry::parse)
            .for_each(|entry| {
                ctx.add(entry);
                ctx.print();

                ctx.stdout.flush().unwrap();
            });
    });

    let stdin = io::stdin();

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Esc) => break,
            _ => {}
        };
    }

    print!("{}", cursor::Show);

    stdout.flush().unwrap();

    Ok(())
}
