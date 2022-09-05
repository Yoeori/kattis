use std::error::Error;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines.next().unwrap().unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    let (r, _, steps) = (first_line[0], first_line[1], first_line[2]);

    let mut found = HashSet::new();
    let mut cur_round: Vec<(isize, isize)> = vec![];
    let delta = vec![(0, 1), (1, 0), (1, 1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (1, -1)];

    for x in 0..r {
        let line = lines.next().unwrap().unwrap();
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                found.insert((x as isize, y as isize));
                cur_round.push((x as isize, y as isize));
            }
        }
    }

    let mut old_count = 0;
    let mut count = cur_round.len();
    let mut prev_diff = 0;
    for i in 0..std::cmp::min(20, steps) {
        let mut next_round = vec![];
        for &(x, y) in &cur_round {
            for &(dx, dy) in &delta {
                if !found.contains(&(x + dx, y + dy)) {
                    found.insert((x + dx, y + dy));
                    next_round.push((x + dx, y + dy));
                    count += 1;
                }
            }
        }

        if i == 19 {
            break;
        }

        prev_diff = count - old_count;
        old_count = count;
        cur_round = next_round;
    }

    if steps > 20 {
        let step_diff = (count - old_count) - prev_diff;
        let mut diff = prev_diff + step_diff * 2;
    
        for _ in 20..steps {
            count += diff;
            diff += step_diff;
        }
    }

    writeln!(&mut w, "{}", count).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gluttonousgoop_sampleinputs() {
        for mut file in std::fs::read_dir("input/gluttonousgoop")
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
