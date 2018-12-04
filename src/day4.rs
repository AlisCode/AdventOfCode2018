use chrono::NaiveDateTime;
use chrono::Timelike;
use fnv::FnvHashMap;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

/// A struct representing a Guard, with its ID
/// and its sleep record
pub struct Guard {
    /// Unique ID of the `Guard`
    id: u32,
    /// Sleep records of the `Guard`
    pub asleep: FnvHashMap<u32, usize>,
}

impl Guard {
    /// Creates a new instance of a `Guard`
    pub fn new(id: u32) -> Guard {
        Guard {
            id,
            asleep: FnvHashMap::default(),
        }
    }

    /// Adds a sleep record from the two given timestamps
    pub fn add_sleep_schedule(&mut self, start: NaiveDateTime, end: NaiveDateTime) {
        let start_min = start.hour() * 60 + start.minute();
        let end_min = end.hour() * 60 + end.minute();
        (start_min..end_min).for_each(|m| {
            let min = self.asleep.entry(m % 60).or_insert(0usize);
            *min += 1;
        });
    }

    /// Merges the given sleep record with the current `Guard`'s
    pub fn append_asleep(&mut self, asleep: FnvHashMap<u32, usize>) {
        asleep.iter().for_each(|(k, v)| {
            let entry = self.asleep.entry(*k).or_insert(0usize);
            *entry += v;
        });
    }

    /// Computes the total sleeping time of a `Guard` in minutes.
    /// Useful for part one.
    pub fn total_sleeping(&self) -> usize {
        self.asleep.values().sum()
    }

    /// Finds out the minute when the `Guard` slept the most using its
    /// sleep record. Returns the minute and the number of times the
    /// `Guard` did sleep at said minute. Apparently, a `Guard`'s sleeping
    /// records can be empty so we need to handle the case of `asleep` being
    /// empty ... I chose to return (0,0).
    pub fn max_sleeping_minute(&self) -> (u32, u32) {
        let (k, v) = self.asleep.iter().max_by_key(|a| a.1).unwrap_or((&0, &0));
        (*k, *v as u32)
    }
}

/// A `GuardBuilder` is a struct implementing a
/// builder pattern for the `Guard` structs, that will
/// create a list of `Guard`s based on given `Instruction`s
pub struct GuardBuilder {
    pub done: Vec<Guard>,
    building: Option<Guard>,
    start_ts: Option<NaiveDateTime>,
}

impl GuardBuilder {
    /// Instantiates a `GuardBuilder` that we will use to
    /// provide a list of `Guard`s
    pub fn new() -> Self {
        GuardBuilder {
            done: vec![],
            building: None,
            start_ts: None,
        }
    }

    /// Adds an instruction to the builder
    pub fn with_instr(self, instr: Instruction) -> Self {
        match instr.action {
            Action::Start(id) => self.handle_start(id),
            Action::Sleep => self.handle_sleep(instr.timestamp),
            Action::Wake => self.handle_wake(instr.timestamp),
        }
    }

    /// Handles the receiving of an `Action::Start`
    fn handle_start(mut self, id: u32) -> Self {
        if let Some(g) = self.building {
            // If we already have an existing `Guard` in the list with
            // the ID of the one being built previously, we must not
            // forget to merge the sleep records
            match self.done.iter_mut().find(|gg| g.id == gg.id) {
                Some(existing) => existing.append_asleep(g.asleep),
                None => self.done.push(g),
            }
        }
        self.start_ts = None;
        self.building = Some(Guard::new(id));
        self
    }

    /// Handles the receiving of an `Action::Sleep`
    fn handle_sleep(mut self, timestamp: NaiveDateTime) -> Self {
        self.start_ts = Some(timestamp);
        self
    }

    /// Handles the receiving of an `Action::Wake`
    fn handle_wake(mut self, timestamp: NaiveDateTime) -> Self {
        // When receiving a Wake signal, we must add the sleep record
        // to the `Guard`'s history
        if let Some(ts) = self.start_ts {
            let mut guard = self.building.unwrap();
            guard.add_sleep_schedule(ts, timestamp);
            self.building = Some(guard);
        }
        self
    }

    /// Finishes the builder, returning a list of `Guard`s structs
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
/// Various actions affecting a `Guard`
pub enum Action {
    /// Starts the shift of one `Guard`
    Start(u32),
    /// One `Guard` goes to sleep
    Sleep,
    /// One `Guard` wakes up
    Wake,
}

#[derive(Eq, PartialEq, Debug)]
/// Instructions as described in the input
pub struct Instruction {
    timestamp: NaiveDateTime,
    pub action: Action,
}

/// PartialOrd and Ord allows me to use .sort()
impl PartialOrd for Instruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// PartialOrd and Ord allows me to use .sort()
impl Ord for Instruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

/// I'm implementing FromStr in order to be able to call .parse, which is more idiomatic
impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = input.split("] ").collect();
        let timestamp: NaiveDateTime = NaiveDateTime::parse_from_str(parts[0], "[%Y-%m-%d %H:%M")?;
        let action: Action = parts[1].trim().parse()?;

        Ok(Instruction { timestamp, action })
    }
}

/// I'm implementing FromStr in order to be able to call .parse, which is more idiomatic
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

/// Generates the guards from the input
#[aoc_generator(day4)]
fn gen_guards(input: &str) -> Vec<Guard> {
    // Parses the instructions from the input
    let mut instructions: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse().expect("Failed to parse instruction"))
        .collect();

    // Sorts the `Instruction`s by Timestamp
    instructions.sort();

    // Uses the `GuardBuilder` to create a list of `Guard` using
    // the ordered `Instruction`s
    instructions
        .into_iter()
        .fold(GuardBuilder::new(), |acc, i| acc.with_instr(i))
        .build()
}

/// Solves the part one
#[aoc(day4, part1)]
fn part_one(input: &[Guard]) -> u32 {
    // Gets the `Guard` that slept the most
    let max_sleeping_guard = input
        .into_iter()
        .max_by_key(|g| g.total_sleeping())
        .expect("Couldn't find max sleeping guard");

    // Computes the minute where said `Guard` slept the most
    let max_sleeping_minute = max_sleeping_guard.max_sleeping_minute();
    max_sleeping_guard.id * max_sleeping_minute.0
}

/// Solves the part two
#[aoc(day4, part2)]
fn part_two(input: &[Guard]) -> u32 {
    // Gets the `Guard` that slept the most on a given minute
    let max_sleeping_guard = input
        .iter()
        .max_by_key(|g| g.max_sleeping_minute().1)
        .expect("Couldn't find max sleeping guard");

    // Computes the minute where said `Guard` slept the most
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
