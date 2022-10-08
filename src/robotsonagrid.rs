use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn is_possible(n: usize, grid: &Vec<Vec<bool>>, (x, y): (usize, usize), marked: &mut Vec<Vec<bool>>) -> bool {
    marked[x][y] = true;
    
    if !grid[x][y] {
        return false;
    }

    if x == n - 1 && y == n - 1 {
        return true;
    }

    if (x + 1 < n && !marked[x + 1][y] && is_possible(n, grid, (x + 1, y), marked)) ||
        (x != 0 && !marked[x - 1][y] && is_possible(n, grid, (x - 1, y), marked)) ||
        (y + 1 < n && !marked[x][y + 1] && is_possible(n, grid, (x, y + 1), marked)) ||
        (y != 0 && !marked[x][y - 1] && is_possible(n, grid, (x, y - 1), marked)) {
        return true;
    }

    return false;
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n: usize = lines.next().unwrap().parse().unwrap();

    let grid: Vec<Vec<bool>> = lines.take(n).map(|x| x.chars().map(|y| y == '.').collect::<Vec<bool>>()).collect();
    let mut count = vec![vec![0u64; n]; n];

    for i in 0..n {
        if grid[0][i] {
            count[0][i] = 1;
        } else {
            break;
        }
    }

    for i in 0..n {
        if grid[i][0] {
            count[i][0] = 1;
        } else {
            break;
        }
    }

    for i in 1..n {
        for p in 0..(n - i) {
            if grid[i + p][i] {
                count[i + p][i] = (count[i + p - 1][i] + count[i + p][i - 1]) % (2u64.pow(31) - 1);
            }

            if grid[i][i + p] {
                count[i][i + p] = (count[i - 1][i + p] + count[i][i + p - 1]) % (2u64.pow(31) - 1);
            }
        }
    }

    if count[n - 1][n - 1] == 0 {
        // Couldn't find path TODO
        if is_possible(n, &grid, (0, 0), &mut vec![vec![false; n]; n]) {
            writeln!(&mut w, "THE GAME IS A LIE").unwrap();
        } else {
            writeln!(&mut w, "INCONCEIVABLE").unwrap();
        }

    } else {
        writeln!(&mut w, "{}", count[n - 1][n - 1]).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robotsonagrid_sampleinputs() {
        for mut file in std::fs::read_dir("input/robotsonagrid")
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
