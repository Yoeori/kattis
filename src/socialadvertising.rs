use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    for _ in 0..lines.next().unwrap().unwrap().parse::<usize>().unwrap() {

        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let friends: Vec<u32> = (&mut lines).take(n).map(|l| l.unwrap()).map(|line| {
            line.split(' ').skip(1).map(|x| 1 << (x.parse::<usize>().unwrap() - 1)).sum()
        }).collect();

        let mut min_friends = n;

        for i in 1..=(1 << n) { // Go through all combinations of friends

            let mut mask = i;
            let mut count = 0;

            for friend in 0..n {
                if (i >> friend) & 1 == 1 {
                    count += 1;
                    mask = mask | friends[friend];
                }
            }

            if mask == (1 << n) - 1 {
                min_friends = std::cmp::min(min_friends, count);
            }
        }

        writeln!(&mut w, "{}", min_friends).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socialadvertising_sampleinputs() {
        for mut file in std::fs::read_dir("input/socialadvertising")
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
