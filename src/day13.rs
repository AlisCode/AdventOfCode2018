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

#[derive(Debug, Clone)]
pub struct Cart {
    dir: i32,
    next_dir: i32,
    x: i32,
    y: i32,
    pub alive: bool,
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
        }
    }

    pub fn tick(&mut self, nodes: &HashMap<(i32, i32), Node>) {
        match self.dir {
            0 => self.x -= 1,
            1 => self.y -= 1,
            2 => self.x += 1,
            3 => self.y += 1,
            _ => unreachable!(),
        }
        let node: &Node = nodes.get(&(self.x, self.y)).expect("Failed to get node!");
        self.handle_node(node);
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
}

impl TracksInfo {
    pub fn tick_all(&mut self) {
        let nodes = &self.nodes;
        self.carts.iter_mut().for_each(|c| c.tick(nodes));
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

    fn clear_colliding_carts(&mut self) {
        let ids: Vec<usize> = self
            .carts
            .iter()
            .enumerate()
            .filter(|(_, c)| c.alive)
            .filter_map(|(i, c)| {
                if self
                    .carts
                    .iter()
                    .filter(|cc| cc.alive && cc.x == c.x && cc.y == c.y)
                    .count()
                    >= 2
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        self.carts
            .iter_mut()
            .enumerate()
            .filter(|(i, c)| c.alive && ids.contains(i))
            .for_each(|(_, mut c)| c.alive = false);

        /*
        let new_carts: Vec<Cart> = self
            .carts
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if self
                    .carts
                    .iter()
                    .enumerate()
                    .all(move |(ii, cc)| ii == i || cc.x != c.x || cc.y != c.y)
                {
                    Some(c.clone())
                } else {
                    None
                }
            })
            .collect();

        self.carts = new_carts;
        */
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

    TracksInfo { nodes, carts }
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
            tracks.tick_all();
            tracks.clear_colliding_carts();

            if tracks.carts.iter().filter(|c| c.alive).count() == 1 {
                let lone_cart = tracks.carts.iter().filter(|c| c.alive).next().unwrap();
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

        assert_eq!(track_info.carts.len(), 2);
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
