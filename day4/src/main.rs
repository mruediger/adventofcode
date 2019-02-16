use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

extern crate chrono;
use chrono::prelude::*;

struct Event {
    date: chrono::DateTime<Utc>,
    event: String,
}

impl Event {
    pub fn new(string: String) -> Event {
        Event {
            date: Utc
                .datetime_from_str(string.get(0..18).unwrap(), "[%Y-%m-%d %H:%M]")
                .unwrap(),
            event: string.get(19..).unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
struct Guard {
    id: u32,
    shift_start: chrono::DateTime<Utc>,
    wakes_up_at: Vec<chrono::DateTime<Utc>>,
    falls_asleep_at: Vec<chrono::DateTime<Utc>>,
}

impl Guard {
    fn new(e: &Event) -> Guard {
        Guard {
            id: e
                .event
                .split(' ')
                .nth(1)
                .and_then(|s| s.get(1..))
                .and_then(|s| s.parse().ok())
                .unwrap(),
            shift_start: e.date,
            wakes_up_at: Vec::new(),
            falls_asleep_at: Vec::new(),
        }
    }

    fn add_event(&mut self, e: &Event) {
        if e.event == "wakes up" {
            self.wakes_up_at.push(e.date)
        }

        if e.event == "falls asleep" {
            self.falls_asleep_at.push(e.date)
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut events: Vec<Event> = BufReader::new(File::open("../input.txt")?)
        .lines()
        .flatten()
        .map(Event::new)
        .collect();

    events.sort_by(|a, b| a.date.cmp(&b.date));

    let mut guards: Vec<Guard> = Vec::new();

    events.iter().for_each(|e| {
        if e.event.starts_with("Guard") {
            guards.push(Guard::new(e))
        } else {
            guards.last_mut().unwrap().add_event(e)
        }
    });

    guards.iter().for_each(|g| println!("{:?}", g));

    Ok(())
}
