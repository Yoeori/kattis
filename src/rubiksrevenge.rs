use std::error::Error;
use std::collections::{VecDeque, HashMap};
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

type Rubik = [u8; 4];

const R: u32 = 0b00;
const G: u32 = 0b01;
const B: u32 = 0b10;
const Y: u32 = 0b11;

const ACTIONS: [fn(&Rubik, usize) -> Rubik; 4] = [
    // Left
    |rubik, i| {
        let mut rubik = rubik.clone();
        rubik[i] = (rubik[i] << 2) + (rubik[i] >> 6);
        rubik
    },
    // Right
    |rubik, i| {
        let mut rubik = rubik.clone();
        rubik[i] = (rubik[i] >> 2) + ((rubik[i] & 0b11) << 6);
        rubik
    },
    // Up
    |rubik, i| {
        let mut rubik = rubik.clone();
        let tmp = rubik[0] & (0b11 << i*2);
        rubik[0] = (rubik[0] & !(0b11 << i*2)) + (rubik[1] & (0b11 << i*2));
        rubik[1] = (rubik[1] & !(0b11 << i*2)) + (rubik[2] & (0b11 << i*2));
        rubik[2] = (rubik[2] & !(0b11 << i*2)) + (rubik[3] & (0b11 << i*2));
        rubik[3] = (rubik[3] & !(0b11 << i*2)) + tmp;
        rubik
    },
    // Down
    |rubik, i| {
        let mut rubik = rubik.clone();
        let tmp = rubik[3] & (0b11 << i*2);
        rubik[3] = (rubik[3] & !(0b11 << i*2)) + (rubik[2] & (0b11 << i*2));
        rubik[2] = (rubik[2] & !(0b11 << i*2)) + (rubik[1] & (0b11 << i*2));
        rubik[1] = (rubik[1] & !(0b11 << i*2)) + (rubik[0] & (0b11 << i*2));
        rubik[0] = (rubik[0] & !(0b11 << i*2)) + tmp;
        rubik
    },

];

#[allow(dead_code)]
fn print_rubik(rubik: &Rubik) {
    println!("Rubik:");
    for i in rubik {
        for o in 0..4 {
            print!("{}", match (i >> (2 * (3 - o)) as i32 & 0b11) as u32 {
                R => 'R',
                G => 'G',
                B => 'B',
                Y => 'Y',
                _ => unreachable!()
            })
        }
        println!("");
    }
}

const GOAL: Rubik = [0b00000000, 0b01010101, 0b10101010, 0b11111111];

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut initial_rubik: [u8; 4] = [0; 4];
    for (i, line) in input.lines().map(|x| x.unwrap()).enumerate().take(4) {
        initial_rubik[i] = line.chars().enumerate().map(|(i, c)| {
            let c = match c {
                'R' => R,
                'G' => G,
                'B' => B,
                'Y' => Y,
                _ => panic!("Unknown letter")
            };
            (c << (2 * (3 - i))) as u8
        }).sum();
    }

    writeln!(&mut w, "{}", find_path(initial_rubik)).unwrap();

    Ok(())
}

fn find_path(initial_rubik: Rubik) -> usize {
    if initial_rubik == GOAL {
        return 0;
    }

    let mut queue_start: VecDeque<Rubik> = VecDeque::new();
    queue_start.push_back(initial_rubik);

    let mut queue_end: VecDeque<Rubik> = VecDeque::new();
    queue_end.push_back(GOAL);

    let mut dist_start: HashMap<Rubik, usize> = HashMap::new();
    let mut dist_end: HashMap<Rubik, usize> = HashMap::new();
    dist_start.insert(initial_rubik, 0);
    dist_end.insert(GOAL, 0);

    while !queue_start.is_empty() && !queue_end.is_empty() {
        let (new_queue, res) = inner_bfs(queue_start, &mut dist_start, &dist_end, GOAL);
        queue_start = new_queue;

        if let Some(res) = res {
            return res;
        }

        let (new_queue, res) = inner_bfs(queue_end, &mut dist_end, &dist_start, initial_rubik);
        queue_end = new_queue;

        if let Some(res) = res {
            return res;
        }
    }

    unreachable!();
}

fn inner_bfs(mut queue: VecDeque<Rubik>, dist: &mut HashMap<Rubik, usize>, other_dist: &HashMap<Rubik, usize>, goal: Rubik) -> (VecDeque<Rubik>, Option<usize>) {
    let mut queue_next = VecDeque::new();
    while let Some(s) = queue.pop_front() {
        let d = *dist.get(&s).unwrap();

        for i in 0..4 {
            for neighbour in ACTIONS.iter().map(|a| a(&s, i)) {
                if neighbour == goal {
                    return (queue_next, Some(d + 1));
                }

                if !dist.contains_key(&neighbour) {
                    queue_next.push_back(neighbour);
                    dist.insert(neighbour, d + 1);
                }

                if other_dist.contains_key(&neighbour) {
                    return (queue_next, Some(dist.get(&neighbour).unwrap() + other_dist.get(&neighbour).unwrap()));
                }
            }
        }
    }

    (queue_next, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rubiksrevenge_sampleinputs() {
        for mut file in std::fs::read_dir("input/rubiksrevenge")
            .unwrap()
            .filter(|f| f.is_ok() && f.as_ref().unwrap().path().extension().unwrap() == "in")
            .map(|f| f.unwrap().path())
        {
            let mut output_writer: Vec<u8> = Vec::new();
            solve(
                std::fs::read_to_string(&file).unwrap().as_bytes(),
                &mut output_writer,
            )
            .unwrap();
            file.set_extension("ans");
            assert_eq!(
                std::str::from_utf8(&output_writer).unwrap().trim(),
                std::fs::read_to_string(&file).unwrap().trim(),
                "file: {:?}",
                file
            );
        }
    }
}
