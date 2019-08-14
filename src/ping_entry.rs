use termion::color::Fg;
use termion::{color, style};

#[derive(Debug, Clone)]
pub struct PingEntry {
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

        format!(
            "{}{}{:>6.1} ms {} {}",
            histo,
            style::Bold,
            self.time,
            self.from_domain,
            self.ttl
        )
    }

    pub fn get_histo_char(time: f32) -> String {
        if time <= 10.0 {
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
            format!("{}\u{2588}", Fg(color::Magenta))
        }
    }

    fn format(&self) -> String {
        let s = self.preformat();

        format!("{}{} {:>5}", style::Reset, s, self.icmp_seq)
    }

    pub fn print(&self) {
        print!("{}", self.format());
    }

    pub fn parse(line: String) -> PingEntry {
        let splitted: Vec<&str> = line.split(" ").collect();

        let bytes = splitted[0].parse().unwrap();
        let from_domain = String::from(splitted[3]);

        let mut i = 4;

        let from_ipv6 = if !splitted[i].contains("icmp") {
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
        let icmp_seq = icmp_seq[1].parse().unwrap();

        i += 1;

        let ttl: Vec<&str> = splitted[i].split("=").collect();
        let ttl = ttl[1].parse().unwrap();

        i += 1;

        let time: Vec<&str> = splitted[i].split("=").collect();
        let time = time[1].parse().unwrap();

        PingEntry {
            bytes,
            from_domain,
            from_ipv6,
            icmp_seq,
            ttl,
            time,
        }
    }
}
