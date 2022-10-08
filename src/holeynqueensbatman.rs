use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

type QueenBoard = [u16; 12];
const EMPTY_BOARD: QueenBoard = [0; 12];

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap()).map(|x| {
        let mut splitter = x.split(' ');
        (splitter.next().unwrap().parse::<usize>().unwrap(), splitter.next().unwrap().parse::<usize>().unwrap())
    });

    let mut boards: Vec<Option<Vec<QueenBoard>>> = vec![None; 12];

    while let Some((n, m)) = lines.next() {
        if n == 0 {
            break;
        }

        if boards[n - 1].is_none() {
            boards[n - 1] = Some(generate_boards(&mut EMPTY_BOARD.clone(), n as i8, 0));
        }

        let holes: Vec<(usize, usize)> = (&mut lines).take(m).collect();

        let mut count = 0;
        'mainloop: for board in boards[n - 1].as_deref().unwrap().iter() {
            for &(row, col) in &holes {
                if board[row] >> col & 1 == 1 {
                    continue 'mainloop;
                }
            }
            count += 1;
        }

        writeln!(&mut w, "{}", count).unwrap();
    }
    
    Ok(())
}

/// Generates board positions, returns count of valid positions
fn generate_boards(cur_board: &mut QueenBoard, size: i8, row: i8) -> Vec<QueenBoard> {
    if row == size {
        return vec![cur_board.clone()];
    }

    let mut result = vec![];
    for col in 0..size {
        if valid_placement(cur_board, size, row, col) {
            cur_board[row as usize] = 1 << col;
            result.append(&mut generate_boards(cur_board, size, row + 1));
            cur_board[row as usize] = 0;
        }
    }

    result
}

fn valid_placement(cur_board: &QueenBoard, size: i8, row: i8, col: i8) -> bool {
    
    // Check column til col
    for i in 0..row {
        if cur_board[i as usize] >> col & 1 == 1 {
            return false;
        }
    }

    // Check diagonal 1 (left top to row/col)
    for i in 1..=(cmp::min(row, col)) {
        if cur_board[(row - i) as usize] >> (col - i) & 1 == 1 {
            return false;
        }
    }

    // Check diagonal 2 (right top to row/col)
    for i in 1..=(cmp::min(row, size - col)) {
        if cur_board[(row - i) as usize] >> (col + i) & 1 == 1 {
            return false;
        }
    }

    // Don't check row since that should be empty anyway, or rows below this row
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holeynqueensbatman_sampleinputs() {
        for mut file in std::fs::read_dir("input/holeynqueensbatman")
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
