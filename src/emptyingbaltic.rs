use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut out: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());

    let first_line = lines.next().unwrap();
    let (h, w) = (first_line[0] as usize, first_line[1] as usize);
    let map: Vec<Vec<isize>> = (&mut lines).take(h).collect();

    let last_line = lines.next().unwrap();
    let (i, j) = (last_line[0] as usize - 1, last_line[1] as usize - 1);

    let mut min_reachable: Vec<Vec<isize>> = vec![vec![0; w]; h];
    min_reachable[i][j] = map[i][j];

    #[derive(Debug, PartialEq, Eq)]
    struct Check {
        loc: (isize, isize),
        depth: isize
    }

    impl Ord for Check {
        fn cmp(&self, other: &Self) -> Ordering { 
            other.depth.cmp(&self.depth).then_with(|| self.loc.cmp(&other.loc))
        }
    }

    impl PartialOrd for Check {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
            Some(self.cmp(other))
         }
    }

    let mut queue: BinaryHeap<Check> = BinaryHeap::new();
    queue.push(Check {
        loc: (i as isize, j as isize),
        depth: map[i][j]
    });

    let neighbours = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, 1), (1, -1)];

    while let Some(Check {
        loc: (x, y),
        depth
    }) = queue.pop() {
        if min_reachable[x as usize][y as usize] < depth {
            continue;
        }
        
        for (dx, dy) in &neighbours {
            if x + dx >= 0 && y + dy >= 0 && x + dx < h as isize && y + dy < w as isize &&
                 min_reachable[x as usize][y as usize] < min_reachable[(x + dx) as usize][(y + dy) as usize]
                 && min_reachable[(x + dx) as usize][(y + dy) as usize] > map[(x + dx) as usize][(y + dy) as usize] {
                min_reachable[(x + dx) as usize][(y + dy) as usize] = std::cmp::max(min_reachable[x as usize][y as usize], map[(x + dx) as usize][(y + dy) as usize]);
                queue.push(Check {
                    loc: (x + dx, y + dy),
                    depth: min_reachable[(x + dx) as usize][(y + dy) as usize]
                });
            }
        }
    }

    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            total += -min_reachable[x][y];
        }
    }

    writeln!(&mut out, "{}", total).unwrap();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptyingbaltic_sampleinputs() {
        for mut file in std::fs::read_dir("input/emptyingbaltic")
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
