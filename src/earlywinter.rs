use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut inputs = input.lines().map(|line| line.unwrap());
    let line1 = inputs.next().unwrap();
    let mut line1 = line1.split(' ');

    let _ = line1.next(); // We don't use n
    let dm: usize = line1.next().unwrap().parse().unwrap();

    let line2 = inputs.next().unwrap();
    let mut line2 = line2.split(' ');

    for counter in 0.. {
        let dmi = line2.next();

        if let Some(dmi) = dmi {
            let dmi: usize = dmi.parse().unwrap();
            if dmi <= dm {
                writeln!(&mut w, "It hadn't snowed this early in {} years!", counter).unwrap();
                break;
            }
        } else {
            writeln!(&mut w, "It had never snowed this early!").unwrap();
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earlywinter_sampleinputs() {
        for mut file in std::fs::read_dir("input/earlywinter")
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
