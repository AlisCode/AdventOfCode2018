use chrono::NaiveDateTime;
use chrono::Timelike;
use fnv::FnvHashMap;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

pub struct Guard {
    id: u32,
    asleep: FnvHashMap<u32, usize>,
}

impl Guard {
    pub fn new(id: u32) -> Guard {
        Guard {
            id,
            asleep: FnvHashMap::default(),
        }
    }

    pub fn add_sleep_schedule(&mut self, start: NaiveDateTime, end: NaiveDateTime) {
        let start_min = start.hour() * 60 + start.minute();
        let end_min = end.hour() * 60 + end.minute();
        (start_min..=end_min).for_each(|m| {
            let min = self.asleep.entry(m % 60).or_insert(0usize);
            *min += 1;
        });
    }

    pub fn total_sleeping(&self) -> usize {
        self.asleep.values().sum()
    }

    pub fn max_sleeping_minute(&self) -> u32 {
        let (k, _) = self
            .asleep
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .expect("Couldnt find max sleeping minute");
        *k
    }
}

pub struct GuardBuilder {
    pub done: Vec<Guard>,
    building: Option<Guard>,
    start_ts: Option<NaiveDateTime>,
}

impl GuardBuilder {
    pub fn new() -> Self {
        GuardBuilder {
            done: vec![],
            building: None,
            start_ts: None,
        }
    }

    pub fn with_instr(self, instr: Instruction) -> Self {
        match instr.action {
            Action::Start(id) => self.handle_start(id),
            Action::Sleep => self.handle_sleep(instr.timestamp),
            Action::Wake => self.handle_wake(instr.timestamp),
        }
    }

    fn handle_start(mut self, id: u32) -> Self {
        if let Some(g) = self.building {
            self.done.push(g);
        }
        self.start_ts = None;
        self.building = Some(Guard::new(id));
        self
    }

    fn handle_sleep(mut self, timestamp: NaiveDateTime) -> Self {
        self.start_ts = Some(timestamp);
        self
    }

    fn handle_wake(mut self, timestamp: NaiveDateTime) -> Self {
        if let Some(ts) = self.start_ts {
            let mut guard = self.building.unwrap();
            guard.add_sleep_schedule(ts, timestamp);
            self.building = Some(guard);
        }
        self
    }

    pub fn build(mut self) -> Vec<Guard> {
        if let Some(g) = self.building {
            self.done.push(g);
        }
        self.done
    }
}

#[derive(Eq, PartialEq)]
pub enum Action {
    Start(u32),
    Sleep,
    Wake,
}

#[derive(Eq, PartialEq)]
pub struct Instruction {
    timestamp: NaiveDateTime,
    pub action: Action,
}

impl PartialOrd for Instruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Instruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = input.split("]").collect();
        let timestamp: NaiveDateTime = NaiveDateTime::parse_from_str(parts[0], "[%Y-%m-%d %H-%M")?;
        let action = parts[1].trim().parse()?;

        Ok(Instruction { timestamp, action })
    }
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        match input {
            s if s.to_string().starts_with("wakes") => Ok(Action::Wake),
            s if s.to_string().starts_with("falls") => Ok(Action::Sleep),
            s if s.to_string().starts_with("Guard") => {
                let parts: Vec<&str> = s.split(" ").collect();
                let guard_id: u32 = parts[1].replace("#", "").parse::<u32>()?;
                Ok(Action::Start(guard_id))
            }
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day4)]
fn gen_guards(input: &str) -> Vec<Guard> {
    let mut instructions: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse().expect("Failed to parse instruction"))
        .collect();

    instructions.sort();

    instructions
        .into_iter()
        .fold(GuardBuilder::new(), |acc, i| acc.with_instr(i))
        .build()
}

#[aoc(day4, part1)]
fn part_one(input: &[Guard]) -> u32 {
    let max_sleeping_guard = input
        .iter()
        .max_by_key(|g| g.total_sleeping())
        .expect("Couldn't find max sleeping guard");

    let max_sleeping_minute = max_sleeping_guard.max_sleeping_minute();
    0
}
