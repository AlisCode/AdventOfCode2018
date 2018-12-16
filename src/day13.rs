use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::AsRef;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Node {
    Switch(bool),
    Crossing,
    Road,
    Empty,
}

impl From<char> for Node {
    fn from(c: char) -> Node {
        match c {
            '/' => Node::Switch(true),
            '\\' => Node::Switch(false),
            '-' | '|' | '>' | 'v' | '<' | '^' => Node::Road,
            '+' => Node::Crossing,
            _ => Node::Empty,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cart {
    dir: i32,
    next_dir: i32,
    x: i32,
    y: i32,
    pub alive: bool,
    pub marked_dead: bool,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            return Ordering::Less;
        } else if self.y == other.y && self.x < other.x {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    pub fn new(x: i32, y: i32, dir: char) -> Self {
        let dir_nb: i32 = match dir {
            '<' => 0,
            '^' => 1,
            '>' => 2,
            'v' => 3,
            _ => unreachable!(),
        };

        Cart {
            x,
            y,
            dir: dir_nb,
            next_dir: 0,
            alive: true,
            marked_dead: false,
        }
    }

    pub fn tick(&mut self, nodes: &HashMap<(i32, i32), Node>) {
        if self.marked_dead {
            self.alive = false;
            self.marked_dead = false;
        }
        if !self.alive {
            return;
        }
        self.advance();
        let node: &Node = nodes
            .get(&(self.x, self.y))
            .expect(&format!("Failed to get node {} {} !", self.x, self.y));
        self.handle_node(node);
    }

    pub fn advance(&mut self) {
        match self.dir {
            0 => self.x -= 1,
            1 => self.y -= 1,
            2 => self.x += 1,
            3 => self.y += 1,
            _ => unreachable!(),
        }
    }

    fn handle_node(&mut self, node: &Node) {
        match *node {
            Node::Crossing => {
                let new_dir = match self.next_dir % 3 {
                    0 => (self.dir + 3) % 4,
                    1 => self.dir,
                    2 => (self.dir + 1) % 4,
                    _ => unreachable!(),
                };
                self.dir = new_dir;
                self.next_dir += 1;
            }
            Node::Switch(dire) => {
                let new_dir = match self.dir {
                    0 if dire => 3,
                    1 if dire => 2,
                    2 if dire => 1,
                    3 if dire => 0,
                    0 if !dire => 1,
                    1 if !dire => 0,
                    2 if !dire => 3,
                    3 if !dire => 2,
                    _ => unreachable!(),
                };
                self.dir = new_dir;
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TracksInfo {
    nodes: HashMap<(i32, i32), Node>,
    carts: Vec<Cart>,
    next: usize,
}

impl TracksInfo {
    pub fn tick_all(&mut self) {
        let nodes = &self.nodes;
        self.carts.iter_mut().for_each(|c| c.tick(nodes));
    }

    pub fn tick_next(&mut self) {
        self.next %= self.carts.len();
        if !self.carts[self.next].alive {
            self.next += 1;
            return;
        }

        self.carts[self.next].tick(&self.nodes);
        let pos = (self.carts[self.next].x, self.carts[self.next].y);
        let carts_collided_ids = self
            .carts
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if c.alive && c.x == pos.0 && c.y == pos.1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        if carts_collided_ids.len() >= 2 {
            carts_collided_ids.into_iter().for_each(|i| {
                self.carts[i].marked_dead = true;
            })
        }

        self.next += 1;
    }

    pub fn sort_carts(&mut self) {
        self.carts.sort_unstable();
    }

    pub fn collision(&self) -> Option<(i32, i32)> {
        self.carts
            .iter()
            .enumerate()
            .map(|(i, c)| {
                self.carts.iter().enumerate().filter_map(move |(ii, cc)| {
                    if ii != i && cc.x == c.x && cc.y == c.y {
                        return Some((cc.x, cc.y));
                    }
                    None
                })
            })
            .flat_map(|i| i.into_iter())
            .next()
    }
}

impl AsRef<TracksInfo> for TracksInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[aoc_generator(day13)]
fn gen_tracks(input: &str) -> TracksInfo {
    let mut carts: Vec<Cart> = vec![];
    let mut nodes: HashMap<(i32, i32), Node> = HashMap::default();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    input.lines().for_each(|l| {
        l.chars().for_each(|c| {
            match c {
                'v' | '^' | '<' | '>' => carts.push(Cart::new(x, y, c)),
                _ => (),
            };
            let n: Node = c.into();
            nodes.insert((x, y), n);
            x += 1;
        });
        y += 1;
        x = 0;
    });

    TracksInfo {
        nodes,
        carts,
        next: 0,
    }
}

#[aoc(day13, part1)]
fn part_one(input: &TracksInfo) -> String {
    let mut tracks = input.clone();

    let first_collision = (0..)
        .filter_map(|_| {
            tracks.tick_all();
            tracks.collision()
        })
        .next()
        .unwrap();

    format!("{},{}", first_collision.0, first_collision.1)
}

#[aoc(day13, part2)]
fn part_two(input: &TracksInfo) -> String {
    let mut tracks = input.clone();

    let pos = (0..)
        .filter_map(|_| {
            tracks.sort_carts();

            (0..tracks.carts.len()).for_each(|_| {
                tracks.tick_next();
            });
            if tracks
                .carts
                .iter()
                .filter(|c| c.alive && !c.marked_dead)
                .count()
                == 1
            {
                let lone_cart = tracks
                    .carts
                    .iter()
                    .filter(|c| c.alive && !c.marked_dead)
                    .next()
                    .unwrap();
                return Some((lone_cart.x, lone_cart.y));
            }

            None
        })
        .next()
        .unwrap();

    format!("{},{}", pos.0, pos.1)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;

    const INPUT_2: &str = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

    #[test]
    fn day13_parsing() {
        let track_info = gen_tracks(INPUT);
        let track_info_2 = gen_tracks(INPUT_2);

        assert_eq!(track_info.carts.len(), 2);
        let cart_one = &track_info.carts[0];
        assert_eq!(cart_one.x, 2);
        assert_eq!(cart_one.y, 0);

        let cart_two = &track_info.carts[1];
        assert_eq!(cart_two.x, 9);
        assert_eq!(cart_two.y, 3);

        let positions = vec![
            (1, 0),
            (3, 0),
            (3, 2),
            (6, 3),
            (1, 4),
            (3, 4),
            (6, 5),
            (3, 6),
            (5, 6),
        ];

        positions.iter().enumerate().for_each(|(i, p)| {
            let cart = &track_info_2.carts[i];
            assert_eq!(cart.x, p.0);
            assert_eq!(cart.y, p.1);
        });

        assert_eq!(
            track_info
                .nodes
                .values()
                .filter(|&a| *a == Node::Crossing)
                .count(),
            4
        );

        assert_eq!(
            track_info
                .nodes
                .values()
                .filter(|&a| *a == Node::Road)
                .count(),
            32
        );

        assert_eq!(
            track_info
                .nodes
                .values()
                .filter(|&a| *a == Node::Switch(true))
                .count(),
            6
        );

        assert_eq!(
            track_info
                .nodes
                .values()
                .filter(|&a| *a == Node::Switch(false))
                .count(),
            6
        );
    }

    #[test]
    fn day13_part_one() {
        let tracks = gen_tracks(INPUT);
        assert_eq!(part_one(&tracks), "7,3".to_string());
    }

    #[test]
    fn day13_part_two() {
        let tracks = gen_tracks(INPUT_2);
        assert_eq!(part_two(&tracks), "6,4".to_string());
    }

}
