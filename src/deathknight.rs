use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut total = 0;
    'outerloop: for _ in 0..n {
        let seq = lines.next().unwrap();
        let mut last_one = false;
        
        for c in seq.chars() {
            match c {
                'C' => {
                    last_one = true;
                }
                'D' => {
                    if last_one {
                        continue  'outerloop;
                    }
                    last_one = false;
                }
                'O' => {
                    last_one = false;
                }
                _ => panic!()
            }
        }

        total += 1;
    }

    writeln!(&mut w, "{}", total).unwrap();


    Ok(())


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deathknight_sampleinputs() {
        for mut file in std::fs::read_dir("input/deathknight")
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
