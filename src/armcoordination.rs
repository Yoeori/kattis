use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
    let first_line: Vec<isize> = lines.next().unwrap();
    let (dx, dy) = (first_line[0], first_line[1]);
    dbg!(dx, dy);

    let radius = lines.next().unwrap()[0];

    // let x = (radius * (std::f64::consts::PI/4f64).sin()) as isize;
    // let y = (radius * (std::f64::consts::PI/4f64).cos()) as isize;

    writeln!(&mut w, "{} {}", dx + radius, dy + radius).unwrap();
    writeln!(&mut w, "{} {}", dx + radius, dy - radius).unwrap();
    writeln!(&mut w, "{} {}", dx - radius, dy - radius).unwrap();
    writeln!(&mut w, "{} {}", dx - radius, dy + radius).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armcoordination_sampleinputs() {
        for mut file in std::fs::read_dir("input/armcoordination")
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
