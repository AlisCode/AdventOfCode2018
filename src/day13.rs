use std::collections::HashMap;

enum Node {
    Switch,
    Crossing,
    Road,
    Empty,
}

impl From<char> for Node {
    fn from(c: char) -> Node {
        match c {
            '/' | '\\' => Node::Switch,
            '-' | '|' | '>' | 'v' | '<' | '^' => Node::Road,
            '+' => Node::Crossing,
            _ => Node::Empty,
        }
    }
}

pub struct Cart {
    dir: i32,
    next_dir: i32,
    x: i32,
    y: i32,
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
        }
    }
}

pub struct TracksInfo {
    nodes: HashMap<(u32, u32), Node>,
    carts: Vec<Cart>,
}

#[aoc_generator(day13)]
fn gen_tracks(input: &str) -> TracksInfo {
    let mut carts: Vec<Cart> = vec![];
    let mut nodes: HashMap<(u32, u32), Node> = HashMap::default();

    let mut x = 0;
    let mut y = 0;
    input.lines().for_each(|l| {
        l.chars().for_each(|c| {
            let cart = match c {
                'v' | '^' | '<' | '>' => Some(Cart::new(x, y, c)),
                _ => None,
            };
            let n: Node = c.into();
            self.nodes.insert((x, y), n);
            x += 1;
        });
        y += 1;
        x = 0;
    });

    TracksInfo { nodes, carts }
}
