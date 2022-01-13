use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut inputs = input.lines().map(|line| line.unwrap().parse::<usize>().unwrap());
    let x = inputs.next().unwrap();
    let n = inputs.next().unwrap();
    let mut remaining = x * (n + 1);

    for _ in 0..n {
        remaining -= inputs.next().unwrap();
    }

    writeln!(&mut w, "{}", remaining).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarifa_sampleinputs() {
        for mut file in std::fs::read_dir("input/tarifa")
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
                std::fs::read_to_string(&file).unwrap().trim()
            );
        }
    }
}
