use std::{cell::RefCell, cmp::Ordering, collections::{BinaryHeap, HashMap}, fmt, io::{self, BufRead, Write, stdout}, rc::Rc};

#[derive(Clone, Copy)]
enum Dir {
    Up, Left, Down, Right
}

impl Dir {
    fn apply(&self, (x, y): (i8, i8)) -> (i8, i8) {
        match self {
            Dir::Up => (x, y+1),
            Dir::Left => (x-1, y),
            Dir::Down => (x, y-1),
            Dir::Right => (x+1, y)
        }
    }

    fn rev(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Left => Dir::Right,
            Dir::Down => Dir::Up,
            Dir::Right => Dir::Left
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dir::Up => write!(f, "up"),
            Dir::Left => write!(f, "left"),
            Dir::Down => write!(f, "down"),
            Dir::Right => write!(f, "right")
        }
    }
}


#[derive(Debug)]
struct Block {
    coords: (i8, i8),
    up: Side,
    left: Side,
    down: Side,
    right: Side
}

impl Block {
    fn set_direction(&mut self, to: Side, dir: &Dir) {
        match dir {
            Dir::Up => self.up = to,
            Dir::Left => self.left = to,
            Dir::Down => self.down = to,
            Dir::Right => self.right = to,
        }
    }

    fn register_walk(&mut self, dir: &Dir, to: Rc<RefCell<Block>>) {
        let cur = match dir {
            Dir::Up => &self.up,
            Dir::Left => &self.left,
            Dir::Down => &self.down,
            Dir::Right => &self.right,
        };

        let new: Side = match cur {
            &Side::Undiscovered | &Side::UnwalkedDoor(_) => {
                Side::WalkedDoor(to)
            }
            _ => panic!("Walked through a wall / discovered door.")
        };

        match dir {
            Dir::Up => self.up = new,
            Dir::Left => self.left = new,
            Dir::Down => self.down = new,
            Dir::Right => self.right = new,
        }
    }

    fn register_enter(&mut self, dir: &Dir, to: Rc<RefCell<Block>>) {
        let dir = dir.rev();
        let cur = match dir {
            Dir::Up => &self.up,
            Dir::Left => &self.left,
            Dir::Down => &self.down,
            Dir::Right => &self.right,
        };

        let new: Side = match cur {
            &Side::WalkedDoor(_) => {
                Side::WalkedDoor(to)
            },
            &Side::Undiscovered | &Side::UnwalkedDoor(_) => {
                Side::UnwalkedDoor(to)
            }
            _ => panic!("Walked through a wall / used used door.")
        };

        match dir {
            Dir::Up => self.up = new,
            Dir::Left => self.left = new,
            Dir::Down => self.down = new,
            Dir::Right => self.right = new,
        }
    }

    fn neighbours(&self) -> [(Dir, &Side); 4] {
        [(Dir::Up, &self.up), (Dir::Left, &self.left), (Dir::Down, &self.down), (Dir::Right, &self.right)]
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            coords: (0, 0),
            up: Side::default(),
            left: Side::default(),
            down: Side::default(),
            right: Side::default()
        }
    }
}

#[derive(Debug)]
enum Side {
    Undiscovered,
    Wall,
    WalkedDoor(Rc<RefCell<Block>>),
    UnwalkedDoor(Rc<RefCell<Block>>)
}

impl Default for Side {
    fn default() -> Self {
        Side::Undiscovered
    }
}

const INITIAL_POSITION: (i8, i8) = (0, 0);

pub fn main() {
    let mut map: HashMap<(i8, i8), Rc<RefCell<Block>>> = HashMap::new();

    // Initial data
    let mut cur_pos = INITIAL_POSITION;
    let mut last_dir = Dir::Left; // the first output, chosen by random diceroll
    map.insert(INITIAL_POSITION, Rc::new(RefCell::new(Block::default())));

    let mut cur_path: Vec<Dir> = Vec::new();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Print the initial direction
    println!("{}", last_dir);
    stdout().flush().expect("Could not write to stdout");

    while let Some(Ok(line)) = lines.next() {
        match &line[..] {
            "wall" => {
                let other_pos = last_dir.apply(cur_pos);
                
                if !map.contains_key(&other_pos) {
                    map.insert(other_pos.clone(), Rc::new(RefCell::new(Block {
                        coords: other_pos.clone(),
                        ..Block::default()
                    })));
                }

                map.get(&cur_pos).unwrap().borrow_mut().set_direction(Side::Wall, &last_dir);
                map.get(&other_pos).unwrap().borrow_mut().set_direction(Side::Wall, &last_dir.rev());
            },
            "ok" => {
                let next_pos = last_dir.apply(cur_pos);
                
                if !map.contains_key(&next_pos) {
                    map.insert(next_pos.clone(), Rc::new(RefCell::new(Block {
                        coords: next_pos.clone(),
                        ..Block::default()
                    })));
                }

                map.get(&cur_pos).unwrap().borrow_mut().register_walk(&last_dir, map.get(&next_pos).unwrap().clone());
                map.get(&next_pos).unwrap().borrow_mut().register_enter(&last_dir, map.get(&cur_pos).unwrap().clone());

                // Set new direction
                cur_pos = next_pos;
            },
            "solved" | "wrong" => {
                // We solved the maze so quit gracefully. / We made a mistake apparently so quit gracefully.
                break;
            },
            _ => {
                // We received unexpected input, the program should panic.
                panic!("Unexpected input received from judge.")
            }
        }

        if cur_path.is_empty() {
            // Calculate new path using dijkstra, note that it will return the path in reverse
            if let Some(path) = dijkstra(cur_pos, &map) {
                cur_path = path;
            } else {
                // There's no path, thus we must conclude that the exit is unreachable.
                println!("no way out");
                stdout().flush().expect("Could not write to stdout");
                continue; // We expect a 'solved' or 'wrong' now
            }
        }

        if let Some(dir) = cur_path.pop() {
            last_dir = dir;
        }

        // Print the next direction
        println!("{}", last_dir);
        stdout().flush().expect("Could not write to stdout");
    }
}

fn dijkstra(from: (i8, i8), map: &HashMap<(i8, i8), Rc<RefCell<Block>>>) -> Option<Vec<Dir>> {
    // Dijkstra
    #[derive(Eq, PartialEq)]
    struct Path {
        cost: u32,
        pos: (i8, i8)
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost).then_with(|| other.pos.cmp(&self.pos))
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut dist: HashMap<(i8, i8), u32> = HashMap::new();
    dist.insert(from, 0);

    let mut prev: HashMap<(i8, i8), (Dir, (i8, i8))> = HashMap::new();

    let mut heap: BinaryHeap<Path> = BinaryHeap::new();
    heap.push(Path {
        cost: 0,
        pos: from
    });

    while let Some(Path { cost, pos }) = heap.pop() {
        let v_dist = *dist.entry(pos.clone()).or_insert(std::u32::MAX);

        if cost > v_dist { continue; }

        'inner: for (dir, side) in &map.get(&pos).unwrap().borrow_mut().neighbours() {
            // Check if found undiscovered
            if let Side::Undiscovered = side {
                let mut path = vec![*dir];
                let mut pos = pos;
                while let Some((prev_dir, prev_pos)) = prev.get(&pos) {
                    path.push(*prev_dir);
                    pos = *prev_pos;
                }
                return Some(path);
            }

            if let Side::Wall | Side::WalkedDoor(_) = side {
                continue 'inner;
            }

            let next_pos = dir.apply(pos);
            if v_dist + 1 < *dist.entry(next_pos.clone()).or_insert(std::u32::MAX) {
                dist.insert(next_pos.clone(), v_dist + 1);
                prev.insert(next_pos.clone(), (*dir, pos));

                heap.push(Path {
                    cost: v_dist + 1,
                    pos: next_pos
                });
            }
        }
    }

    None
}