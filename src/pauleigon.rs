use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let input: Vec<usize> = input.lines().next().unwrap().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();

    if (input[1] + input[2]) % (2 * input[0]) < input[0] {
        writeln!(&mut w, "paul").unwrap();
    } else {
        writeln!(&mut w, "opponent").unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pauleigon_sampleinputs() {
        for mut file in std::fs::read_dir("input/pauleigon")
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
