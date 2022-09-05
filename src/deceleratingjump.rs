use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
    let n = lines.next().unwrap()[0] as usize;
    let squares = lines.next().unwrap();

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct VisibleRange {
        cur_score: isize,
        longest_possible_jump: usize
    }

    impl Ord for VisibleRange {
        fn cmp(&self, other: &Self) -> Ordering { 
            self.cur_score.cmp(&other.cur_score).then_with(|| self.longest_possible_jump.cmp(&other.longest_possible_jump))
        }
    }

    impl PartialOrd for VisibleRange {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
            Some(self.cmp(other))
         }
    }

    let mut score = vec![vec![-std::isize::MAX; n]; n];
    score[0][n - 1] = squares[0];

    let mut visible_ranges = vec![BinaryHeap::<VisibleRange>::new(); n];
    visible_ranges[0].push(VisibleRange {
        cur_score: squares[0],
        longest_possible_jump: n - 1
    });

    for pos in 0..n-1 {
        let mut max = 1;
        while let Some(VisibleRange { cur_score, longest_possible_jump }) = visible_ranges[pos].pop() {
            for jump_length in max..=std::cmp::min(n - pos - 1, longest_possible_jump) {
                if score[pos + jump_length][jump_length] < cur_score + squares[pos + jump_length] {
                    score[pos + jump_length][jump_length] = cur_score + squares[pos + jump_length];
                    visible_ranges[pos + jump_length].push(VisibleRange {
                        cur_score: score[pos + jump_length][jump_length],
                        longest_possible_jump: jump_length
                    });
                }
            }
            max = std::cmp::max(max, longest_possible_jump);
        }
    }

    let final_place = visible_ranges[n - 1].pop().unwrap();
    writeln!(&mut w, "{}", final_place.cur_score).unwrap();

    Ok(())

    // Solve idea: Longest path in dag where vertex contains position and longest possible jump;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deceleratingjump_sampleinputs() {
        for mut file in std::fs::read_dir("input/deceleratingjump")
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
