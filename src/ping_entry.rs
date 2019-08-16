use termion::color::{Bg, Fg};
use termion::{color, style};

#[derive(Debug, Clone)]
pub enum EntryType {
    Title(String),
    Pong(String),
    Error(String),
}

impl EntryType {
    pub fn get_inner(&self) -> String {
        match self.clone() {
            EntryType::Title(s) => s,
            EntryType::Pong(s) => s,
            EntryType::Error(s) => s,
        }
    }
}

impl Default for EntryType {
    fn default() -> EntryType {
        EntryType::Error("DEFAULT".to_string())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PingEntry {
    pub t: EntryType,
    pub bytes: u8,
    pub from_domain: String,
    pub from_ipv6: String,
    pub icmp_seq: u64,
    pub ttl: u16,
    pub time: f32,
}

impl PingEntry {
    fn preformat(&self) -> String {
        let histo = Self::get_histo_char(self.time);

        format!("{} {}", histo, self.t.get_inner(),)
    }

    pub fn get_histo_char(time: f32) -> String {
        if time <= 0.0 {
            format!("{}{}X", Bg(color::Red), Fg(color::White))
        } else if time <= 10.0 {
            format!("{}\u{2581}", Fg(color::LightBlue))
        } else if time <= 20.0 {
            format!("{}\u{2582}", Fg(color::Cyan))
        } else if time <= 50.0 {
            format!("{}\u{2583}", Fg(color::LightGreen))
        } else if time <= 100.0 {
            format!("{}\u{2584}", Fg(color::LightYellow))
        } else if time <= 200.0 {
            format!("{}\u{2585}", Fg(color::Yellow))
        } else if time <= 500.0 {
            format!("{}\u{2586}", Fg(color::LightRed))
        } else if time <= 1000.0 {
            format!("{}\u{2587}", Fg(color::Red))
        } else {
            format!("{}\u{2588}", Fg(color::LightMagenta))
        }
    }

    fn format(&self) -> String {
        let s = self.preformat();

        format!("{}{}", style::Reset, s)
    }

    pub fn print(&self) {
        match self.t {
            EntryType::Title(_) => (),
            EntryType::Pong(_) => print!("{}", self.format()),
            EntryType::Error(_) => print!("{}", self.format()),
        }
    }

    pub fn parse(line: String) -> PingEntry {
        let mut res = PingEntry::default();

        let splitted: Vec<&str> = line.split(" ").collect();

        if splitted[0] == "PING" {
            res.t = EntryType::Title(line.clone());

            return res;
        }
        if splitted[0].parse::<i32>().is_err() {
            res.t = EntryType::Error(line.clone());

            return res;
        }

        res.t = EntryType::Pong(line.clone());

        res.bytes = splitted[0].parse().unwrap();
        res.from_domain = String::from(splitted[3]);

        let mut i = 4;

        res.from_ipv6 = if !splitted[i].contains("icmp") {
            let mut from_ipv6 = String::from(splitted[i]);
            from_ipv6.remove(from_ipv6.len() - 1);
            from_ipv6.remove(from_ipv6.len() - 1);
            from_ipv6.remove(0);
            i += 1;
            from_ipv6
        } else {
            String::from("")
        };

        let icmp_seq: Vec<&str> = splitted[i].split("=").collect();
        res.icmp_seq = icmp_seq[1].parse().unwrap();

        i += 1;

        let ttl: Vec<&str> = splitted[i].split("=").collect();
        res.ttl = ttl[1].parse().unwrap();

        i += 1;

        let time: Vec<&str> = splitted[i].split("=").collect();
        res.time = time[1].parse().unwrap();

        res
    }
}
