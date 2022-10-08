use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n = lines.next().unwrap().parse().unwrap();


    let scores: Vec<(usize, usize)> = lines.take(n).map(|x| {
        let mut x = x.split('-');
        (x.next().unwrap().parse::<usize>().unwrap(), x.next().unwrap().parse::<usize>().unwrap())
    }).collect();

    let mut haswon = false;

    let mut alice = 0;
    let mut bob = 0;

    for (i, &(s0, s1)) in scores.iter().enumerate() {
        // First we check whose round it is:
        let whom = ((s0 + s1 + 1) / 2) % 2;
        
        if whom == 0 {
            if haswon && (s0 != alice || s1 != bob) {
                writeln!(&mut w, "error {}", i + 1).unwrap();
                return Ok(());
            }

            if s0 < alice || s1 < bob {
                writeln!(&mut w, "error {}", i + 1).unwrap();
                return Ok(());
            }

            alice = s0;
            bob = s1;
        } else {
            if haswon && (s1 != alice || s0 != bob) {
                writeln!(&mut w, "error {}", i + 1).unwrap();
                return Ok(());
            }

            if s1 < alice || s0 < bob {
                writeln!(&mut w, "error {}", i + 1).unwrap();
                return Ok(());
            }

            alice = s1;
            bob = s0;
        }

        haswon = s0 == 11 || s1 == 11;

    }

    if haswon && alice == bob {
        writeln!(&mut w, "error {}", scores.len()).unwrap();
        return Ok(());
    }

    writeln!(&mut w, "ok").unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acearbiter_sampleinputs() {
        for mut file in std::fs::read_dir("input/acearbiter")
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
