use std::cmp::Ordering;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub name: u8,
    depends: Vec<u8>,
}

impl Node {
    pub fn add_dependency(&mut self, dep: u8) {
        self.depends.push(dep);
    }

    pub fn new(name: u8, first_dep: u8) -> Self {
        let depends = vec![first_dep];
        Node { name, depends }
    }

    pub fn new_empty(name: u8) -> Self {
        Node {
            name,
            depends: vec![],
        }
    }

    pub fn deps_ok(&self, solved: &[u8]) -> bool {
        self.depends.len() == 0 || self.depends.iter().all(|d| solved.iter().any(|i| d == i))
    }

    pub fn duration(&self, additional_time: u8) -> u32 {
        (self.name - 'A' as u8 + additional_time + 1).into()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dependency {
    pub name: u8,
    pub depends_on: u8,
}

impl FromStr for Dependency {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = input.trim().split(" ").collect();
        Ok(Dependency {
            name: parts[7].as_bytes()[0],
            depends_on: parts[1].as_bytes()[0],
        })
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct WorkerManager {
    nodes: Vec<Node>,
    workers: Vec<Option<(u8, u32)>>,
    additional_time: u8,
    duration: u32,
}

impl WorkerManager {
    pub fn new(nodes: Vec<Node>, nb_workers: u32, additional_time: u8) -> Self {
        let workers: Vec<Option<(u8, u32)>> = (1..=nb_workers).map(|_| None).collect();
        WorkerManager {
            nodes,
            workers,
            additional_time,
            duration: 0,
        }
    }

    pub fn solve(&mut self) {
        let mut solved: Vec<u8> = vec![];
        let target = self.nodes.len();
        while solved.len() != target {
            for w in self.workers.iter_mut().filter(|w| w.is_none()) {
                if let Some(node) = self.nodes.iter().find(|n| n.deps_ok(&solved)) {
                    *w = Some((node.name, node.duration(self.additional_time)));
                    let idx_first = self
                        .nodes
                        .iter()
                        .position(|n| n.name == node.name)
                        .expect("Failed to find node ...");
                    self.nodes.remove(idx_first);
                }
            }

            for w in self.workers.iter_mut().filter(|w| w.is_some()) {
                *w = Some((w.unwrap().0, w.unwrap().1 - 1));
                if w.unwrap().1 == 0 {
                    solved.push(w.unwrap().0);
                    *w = None;
                }
            }

            self.duration += 1;
        }
    }
}

#[aoc_generator(day7)]
pub fn gen_nodes(input: &str) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];

    input
        .lines()
        .map(|l| l.parse::<Dependency>().expect("Failed to parse Dependency"))
        .for_each(|d| match nodes.iter_mut().find(|n| n.name == d.name) {
            Some(n) => n.add_dependency(d.depends_on),
            _ => {
                nodes.push(Node::new(d.name, d.depends_on));
                if nodes.iter().filter(|n| n.name == d.depends_on).count() == 0 {
                    nodes.push(Node::new_empty(d.depends_on));
                }
            }
        });

    nodes.sort();
    nodes
}

#[aoc(day7, part1)]
pub fn part_one(input: &[Node]) -> String {
    let mut nodes: Vec<Node> = input.to_vec();
    let mut solved: Vec<u8> = vec![];
    let mut order: String = "".into();
    while solved.len() < input.len() {
        let first = nodes
            .iter()
            .position(|n| n.deps_ok(&solved))
            .expect("Failed to find node ...");

        let node = &nodes[first];
        order.push(node.name as char);
        solved.push(node.name);
        nodes.remove(first);
    }
    order
}

#[aoc(day7, part2)]
pub fn part_two(input: &[Node]) -> u32 {
    calc_duration(input, 5, 60)
}

fn calc_duration(input: &[Node], workers: u32, additional_time: u8) -> u32 {
    let mut worker_manager = WorkerManager::new(input.to_vec(), workers, additional_time);
    worker_manager.solve();
    worker_manager.duration
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_TEST: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn day7_parse_dep() {
        assert_eq!(
            "Step C must be finished before step A can begin."
                .parse::<Dependency>()
                .expect("Failed to parse dependency"),
            Dependency {
                name: 'A' as u8,
                depends_on: 'C' as u8
            }
        )
    }

    #[test]
    fn day7_part_one() {
        let nodes = gen_nodes(INPUT_TEST);
        assert_eq!(part_one(&nodes), "CABDFE");
    }

    #[test]
    fn day7_part_two() {
        let nodes = gen_nodes(INPUT_TEST);
        assert_eq!(calc_duration(&nodes, 2, 0), 15)
    }

}
