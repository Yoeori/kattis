use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    for line in input.lines() {
        let input = format!("{0}{0}", line.unwrap());
        let mut shifts: Vec<&str> = (0..(input.len() / 2)).into_iter().map(|x| &input[x..((input.len() / 2) + x)]).collect();
        shifts.sort();
        for shift in shifts {
            write!(&mut w, "{}", &shift[(input.len() / 2) - 1..]).unwrap();
        }
        writeln!(&mut w).unwrap();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burrowswheeler_sampleinputs() {
        for mut file in std::fs::read_dir("input/burrowswheeler")
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
                std::str::from_utf8(&output_writer).unwrap(),
                std::fs::read_to_string(&file).unwrap(),
                "file: {:?}", &file
            );
        }
    }
}
