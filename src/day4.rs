use chrono::NaiveDateTime;
use chrono::Timelike;
use fnv::FnvHashMap;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

pub struct Guard {
    id: u32,
    pub asleep: FnvHashMap<u32, usize>,
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
        (start_min..end_min).for_each(|m| {
            let min = self.asleep.entry(m % 60).or_insert(0usize);
            *min += 1;
        });
    }

    pub fn append_asleep(&mut self, asleep: FnvHashMap<u32, usize>) {
        asleep.iter().for_each(|(k, v)| {
            let entry = self.asleep.entry(*k).or_insert(0usize);
            *entry += v;
        });
    }

    pub fn total_sleeping(&self) -> usize {
        self.asleep.values().sum()
    }

    pub fn max_sleeping_minute(&self) -> (u32, u32) {
        let (k, v) = self.asleep.iter().max_by_key(|a| a.1).unwrap_or((&0, &0));
        (*k, *v as u32)
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
            match self.done.iter_mut().find(|gg| g.id == gg.id) {
                Some(existing) => existing.append_asleep(g.asleep),
                None => self.done.push(g),
            }
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
            match self.done.iter_mut().find(|gg| g.id == gg.id) {
                Some(existing) => existing.append_asleep(g.asleep),
                None => self.done.push(g),
            }
        }
        self.done
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Action {
    Start(u32),
    Sleep,
    Wake,
}

#[derive(Eq, PartialEq, Debug)]
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
        let parts: Vec<&str> = input.split("] ").collect();
        let timestamp: NaiveDateTime = NaiveDateTime::parse_from_str(parts[0], "[%Y-%m-%d %H:%M")?;
        let action: Action = parts[1].trim().parse()?;

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
                let guard_id: u32 = parts[1].replace("#", "").trim().parse::<u32>()?;
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
        .into_iter()
        .max_by_key(|g| g.total_sleeping())
        .expect("Couldn't find max sleeping guard");

    let max_sleeping_minute = max_sleeping_guard.max_sleeping_minute();
    max_sleeping_guard.id * max_sleeping_minute.0
}

#[aoc(day4, part2)]
fn part_two(input: &[Guard]) -> u32 {
    let test = input.iter().count();
    let max_sleeping_guard = input
        .iter()
        .max_by_key(|g| g.max_sleeping_minute().1)
        .expect("Couldn't find max sleeping guard");

    let max_sleeping_minute = max_sleeping_guard.max_sleeping_minute();
    max_sleeping_guard.id * max_sleeping_minute.0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_parse_actions() {
        assert_eq!(
            "Guard #10 begins shift"
                .parse::<Action>()
                .expect("Failed to parse"),
            Action::Start(10)
        );
        assert_eq!(
            "falls asleep".parse::<Action>().expect("Failed to parse"),
            Action::Sleep
        );
        assert_eq!(
            "wakes up".parse::<Action>().expect("Failed to parse"),
            Action::Wake
        );
    }

    #[test]
    fn test_parse_instructions() {
        let instr = "[1518-11-01 00:00] Guard #10 begins shift"
            .parse::<Instruction>()
            .expect("Failed to parse");
        assert_eq!(instr.action, Action::Start(10));
    }

    #[test]
    fn test_most_slept_minute() {
        let guards = gen_guards(TEST_INPUT);
        let guard_10 = guards
            .iter()
            .find(|g| g.id == 10)
            .expect("No guard 10 found");
        assert_eq!(guard_10.max_sleeping_minute().0, 24);
        assert_eq!(guard_10.max_sleeping_minute().1, 2);

        let guard_99 = guards
            .iter()
            .find(|g| g.id == 99)
            .expect("No guard 99 found");
        assert_eq!(guard_99.max_sleeping_minute().0, 45);
        assert_eq!(guard_99.max_sleeping_minute().1, 3);
    }

    #[test]
    fn test_part_one() {
        let guards = gen_guards(TEST_INPUT);
        assert_eq!(part_one(&guards), 240);
    }

    #[test]
    fn test_part_two() {
        let guards = gen_guards(TEST_INPUT);
        assert_eq!(part_two(&guards), 4455);
    }
}
