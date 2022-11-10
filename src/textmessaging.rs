use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    for i in 0..(lines.next().unwrap().parse().unwrap()) {
        let line = lines.next().unwrap();
        let mut line = line.split(' ').map(|x| x.parse::<usize>().unwrap());
        let (_, k, _) = (line.next().unwrap(), line.next().unwrap(), line.next().unwrap());
        let mut freq: Vec<usize> = lines.next().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
        freq.sort();
        freq.reverse();

        let mut total = 0;

        let mut available = k;
        let mut depth = 1;

        for presses in freq {
            if available == 0 {
                depth += 1;
                available = k;
            }

            available -= 1;
            total += depth * presses;
        }

        writeln!(&mut w, "Case #{}: {}", i + 1, total).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textmessaging_sampleinputs() {
        for mut file in std::fs::read_dir("input/textmessaging")
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
