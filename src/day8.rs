use std::convert::AsRef;

pub struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        self
    }
}

impl Node {
    pub fn new(chars: &mut impl Iterator<Item = u32>) -> Self {
        let header = (
            chars.next().expect("Failed to get child nodes nb") as usize,
            chars.next().expect("Failed to get metadata nb") as usize,
        );
        let children: Vec<Node> = (0..header.0).map(|_| Node::new(chars)).collect();
        let metadata: Vec<u32> = (0..header.1).filter_map(|_| chars.next()).collect();
        Node { children, metadata }
    }

    pub fn sum(&self) -> u32 {
        self.children.iter().map(|c| c.sum()).sum::<u32>() + self.metadata.iter().sum::<u32>()
    }

    pub fn value(&self) -> u32 {
        if self.children.len() == 0 {
            return self.metadata.iter().sum::<u32>();
        }

        self.metadata
            .iter()
            .filter_map(|m| match (*m as usize).checked_sub(1) {
                Some(i) => self.children.get(i),
                _ => None,
            })
            .map(|c| c.value())
            .sum()
    }
}

#[aoc_generator(day8)]
fn gen_node(input: &str) -> Node {
    let mut chars = input
        .trim()
        .split(" ")
        .map(|s| s.parse().expect("Failed to read u32"));
    Node::new(&mut chars)
}

#[aoc(day8, part1)]
fn part_one(root: &Node) -> u32 {
    root.sum()
}

#[aoc(day8, part2)]
fn part_two(root: &Node) -> u32 {
    root.value()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn day8_tree_parsing() {
        let a = gen_node(INPUT);
        assert_eq!(a.children.len(), 2);
        assert_eq!(a.metadata.len(), 3);
        assert_eq!(a.metadata, vec![1, 1, 2]);

        let b = &a.children[0];
        let c = &a.children[1];

        assert_eq!(b.children.len(), 0);
        assert_eq!(b.metadata.len(), 3);
        assert_eq!(b.metadata, vec![10, 11, 12]);

        assert_eq!(c.children.len(), 1);
        assert_eq!(c.metadata.len(), 1);
        assert_eq!(c.metadata, vec![2]);

        let d = &c.children[0];

        assert_eq!(d.children.len(), 0);
        assert_eq!(d.metadata.len(), 1);
        assert_eq!(d.metadata, vec![99]);
    }

    #[test]
    fn day8_part_one() {
        let root = gen_node(INPUT);
        assert_eq!(part_one(&root), 138);
    }

    #[test]
    fn day8_part_two() {
        let root = gen_node(INPUT);
        assert_eq!(part_two(&root), 66);
    }
}
