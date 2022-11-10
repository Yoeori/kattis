use std::cmp;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

const DIRS: &[(isize, isize)] = &[(0, -1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, 1), (1, -1), (-1, -1)];

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let first_line = lines.next().unwrap();
    let mut first_line = first_line.split(' ').map(|x| x.parse::<usize>().unwrap());
    let (r, s) = (first_line.next().unwrap(), first_line.next().unwrap());

    let mut grid: Vec<Vec<bool>> = vec![vec![false; s + 2]; r + 2];
    for row in 0..r {
        for (i, c) in lines.next().unwrap().chars().enumerate() {
            if c == 'o' {
                grid[row + 1][i + 1] = true;
            }
        }
    }

    // We can for each place the amount of handshakes: possible (and denote max possible), and actual handshakes
    let mut total_handshakes = 0;
    let mut max_extra_handshakes = 0;

    for x in 1..(r + 1) {
        for y in 1..(s + 1) {
            if grid[x][y] {
                for (dx, dy) in DIRS {
                    if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] {
                        total_handshakes += 1;
                    }
                }
            } else {
                let mut possible_here = 0;
                for (dx, dy) in DIRS {
                    if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] {
                        possible_here += 1;
                    }
                }

                max_extra_handshakes = cmp::max(max_extra_handshakes, possible_here);
            }
        }
    }

    writeln!(&mut w, "{}", (total_handshakes / 2) + max_extra_handshakes).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misa_sampleinputs() {
        for mut file in std::fs::read_dir("input/misa")
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
