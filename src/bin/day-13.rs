use::std::collections::{HashSet, VecDeque};
use derive_more::{Add};
use aoc::utils::*;
use std::cmp::Ordering;

#[derive(Default, Clone, Add, Eq, PartialEq, Debug, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Cart {
    position: Position,
    actions: VecDeque<char>,
    // directions: VecDeque<char>,
    moves: VecDeque<Position>,
    alive: bool,
}

impl From<(char, i32, i32)> for Cart {
    fn from(direction_xy: (char, i32, i32)) -> Self {
        let mut directions: VecDeque<char> = vec!['^', '<', 'v', '>'].into();
        let mut cart = Cart {
            position: Position{x: direction_xy.1, y: direction_xy.2},
            actions: vec!['l', 's', 'r'].into(),
            // directions: vec!['^', '<', 'v', '>'].into(),
            moves: vec![
                Position{x: 0, y: -1},
                Position{x: -1, y: 0},
                Position{x: 0, y: 1},
                Position{x: 1, y: 0}
            ].into(),
            alive: true,
        };
        while direction_xy.0 != directions[0] {
            directions.rotate_left(1);
            cart.moves.rotate_left(1);
        }
        cart
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.position.y, self.position.x).cmp(&(other.position.y, other.position.x))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cart_from () {
        assert_eq!(
            Cart::from(('v', 4, 7)),
            Cart{
                position: Position{x: 4, y: 7},
                actions: vec!['l', 's', 'r'].into(),
                // directions: vec!['v', '>', '^', '<'].into(),
                moves: vec![
                    Position{x: 0, y: 1},
                    Position{x: 1, y: 0},
                    Position{x: 0, y: -1},
                    Position{x: -1, y: 0}
                ].into(),
                alive: true,
            }
        )
    }
    #[test]
    fn test_cart_move () {
        let mut cart = Cart::from(('v', 4, 7));
        let mut cart_ref = cart.clone();
        cart.do_move('|');
        cart_ref.position.y += 1;
        assert_eq!(cart, cart_ref);
        cart.do_move('+');
        cart_ref.position.x += 1;
        cart_ref.actions = vec!['s', 'r', 'l'].into();
        cart_ref.moves = vec![
            Position{x: 1, y: 0},
            Position{x: 0, y: -1},
            Position{x: -1, y: 0},
            Position{x: 0, y: 1},
        ].into();
        assert_eq!(cart, cart_ref);
        cart.do_move('/');
        cart_ref.position.y -= 1;
        cart_ref.moves = vec![
            Position{x: 0, y: -1},
            Position{x: -1, y: 0},
            Position{x: 0, y: 1},
            Position{x: 1, y: 0},
        ].into();
        assert_eq!(cart, cart_ref);
        cart.do_move('\\');
        cart_ref.position.x -= 1;
        cart_ref.moves = vec![
            Position{x: -1, y: 0},
            Position{x: 0, y: 1},
            Position{x: 1, y: 0},
            Position{x: 0, y: -1},
        ].into();
        assert_eq!(cart, cart_ref);
        cart.do_move('\\');
        cart_ref.position.y -= 1;
        cart_ref.moves = vec![
        Position{x: 0, y: -1},
            Position{x: -1, y: 0},
            Position{x: 0, y: 1},
            Position{x: 1, y: 0},
        ].into();
        assert_eq!(cart, cart_ref);
        cart.do_move('/');
        cart_ref.position.x += 1;
        cart_ref.moves = vec![
            Position{x: 1, y: 0},
            Position{x: 0, y: -1},
            Position{x: -1, y: 0},
            Position{x: 0, y: 1},
        ].into();
        assert_eq!(cart, cart_ref);
    }
    #[test]
    fn test_cart_order () {
        assert_eq!(Cart::from(('v', 4, 8)) > Cart::from(('v', 3, 7)), true);
        assert_eq!(Cart::from(('v', 3, 7)) == Cart::from(('v', 3, 7)), true);
        assert_eq!(Cart::from(('v', 3, 8)) > Cart::from(('v', 3, 7)), true);
        assert_eq!(Cart::from(('v', 4, 7)) > Cart::from(('v', 3, 7)), true);
    }
    #[test]
    fn test_find_collision() {
        assert_eq!(
            find_collision(vec![
                r"/->-\        ".into(),
                r"|   |  /----\".into(),
                r"| /-+--+-\  |".into(),
                r"| | |  | v  |".into(),
                r"\-+-/  \-+--/".into(),
                r"  \------/   ".into()
            ]),
            Position{x: 7, y: 3}
        );
        // assert_eq!(1, 0);
    }
    #[test]
    fn test_elimination() {
        assert_eq!(
            eliminate_carts(vec![
                r"/>-<\  ".into(),
                r"|   |  ".into(),
                r"| /<+-\".into(),
                r"| | | v".into(),
                r"\>+</ |".into(),
                r"  |   ^".into(),
                r"  \<->/".into()
            ]),
            Position{x: 6, y: 4}
        );
        // assert_eq!(1, 0);
    }
}

impl Cart {
    fn do_move (&mut self, track: char) -> Position {
        let mut turn = 's';
        match track {
            '+' => {
                turn = self.actions[0];
                self.actions.rotate_left(1);
            },
            '\\' => {
                match self.moves[0] {
                    Position{x: 1, y: 0} | Position{x: -1, y: 0} => turn = 'r',
                    _ => turn = 'l',
                }
            },
            '/' => {
                match self.moves[0] {
                    Position{x: 1, y: 0} | Position{x: -1, y: 0} => turn = 'l',
                    _ => turn = 'r',
                }
            },
            _ => (),
        }
        match turn {
            'l' => self.moves.rotate_left(1),
            'r' => self.moves.rotate_right(1),
            _ => ()
        }
        (*self).position = self.position.clone() + self.moves[0].clone();
        self.position.clone()
    }
}

fn print_carts (mut tracks: Vec<Vec<char>>, carts: &Vec<Cart>) {
    for cart in carts {
        tracks[cart.position.y as usize][cart.position.x as usize] = match cart.moves[0] {
            Position{x: 0, y: 1} => 'v',
            Position{x: 0, y: -1} => '^',
            Position{x: 1, y: 0} => '>',
            Position{x: -1, y: 0} => '<',
            _ => tracks[cart.position.y as usize][cart.position.x as usize]
        }
    }
    for line in tracks.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>() {
        println!("{:?}", line);
    }
    println!("")
}

fn find_collision(def: Vec<String>) -> Position {
    let mut tracks: Vec<Vec<char>> = def.iter().map(|v| v.chars().collect()).collect();
    let mut carts = Vec::new();
    let directions = vec!['^', '>', 'v', '<'];
    for i in 0 .. tracks.len() {
        for j in 0 .. tracks[i].len() {
            if directions.contains(&tracks[i][j]) {
                carts.push(Cart::from((tracks[i][j], j as i32, i as i32)));
                match tracks[i][j] {
                    '^' | 'v' => tracks[i][j] = '|',
                    '<' | '>' => tracks[i][j] = '-',
                    _ => ()
                }
            }
        }
    }
    // println!("{:?}", tracks);
    // print_carts(tracks.clone(), &carts);
    let mut positions: HashSet<Position> = HashSet::with_capacity(carts.len());
    let mut position = Position::default();
    let mut iters: usize = 0;
    loop {
        positions.clear();
        for i in 0 .. carts.len() {
            let track = tracks[carts[i].position.y as usize][carts[i].position.x as usize];
            positions.insert(carts[i].do_move(track));
            if positions.len() < i + 1 {
                position = carts[i].position.clone();
                break;
            }
        }
        // print_carts(tracks.clone(), &carts);
        if position != Position::default() {
            break;
        }
        iters += 1;
        if iters > 10000 {
            panic!("{} iterations reached w/o collisions", iters);
        }
    }
    position
}

fn eliminate_carts(def: Vec<String>) -> Position {
    let mut tracks: Vec<Vec<char>> = def.iter().map(|v| v.chars().collect()).collect();
    let mut carts = Vec::new();
    let directions = vec!['^', '>', 'v', '<'];
    for i in 0 .. tracks.len() {
        for j in 0 .. tracks[i].len() {
            if directions.contains(&tracks[i][j]) {
                carts.push(Cart::from((tracks[i][j], j as i32, i as i32)));
                match tracks[i][j] {
                    '^' | 'v' => tracks[i][j] = '|',
                    '<' | '>' => tracks[i][j] = '-',
                    _ => ()
                }
            }
        }
    }
    // println!("{:?}", tracks);
    // print_carts(tracks.clone(), &carts);
    let mut iters: usize = 0;
    while carts.iter().filter(|&c| c.alive).count() > 1 {
        carts.sort();
        for i in 0 .. carts.len() {
            if carts[i].alive {
                let track = tracks[carts[i].position.y as usize][carts[i].position.x as usize];
                carts[i].do_move(track);
                for j in 0 .. carts.len() {
                    if carts[i].position == carts[j].position && carts[j].alive && i != j {
                        println!("Remove {} + {}: {:?} == {:?}", i, j, carts[j], carts[i]);
                        carts[i].alive = false;
                        carts[j].alive = false;
                    }
                }
            }
        }
        iters += 1;
        if iters > 100000 {
            panic!("{} iterations reached w/o collisions", iters);
        }
    }
    (*carts.iter().filter(|&c| c.alive).map(|c| c.position.clone()).collect::<Vec<Position>>().last().unwrap()).clone()
}

fn main () {
    let definition: Vec<String> = read_inputs("inputs/day-13.txt");
    println!("First collision = {:?}", find_collision(definition.clone()));
    println!("Last cart standing = {:?}", eliminate_carts(definition.clone()));
}
