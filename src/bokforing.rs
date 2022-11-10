use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let first_line: Vec<usize> = lines.next().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
    let (n, q) = (first_line[0], first_line[1]);

    let mut people = vec![(0, -1); n];

    let mut last_restart = 0;
    let mut last_restart_v = 0;
    for i in 0..q {
        let line = lines.next().unwrap();
        let mut iter = line.split(' ');

        let v0 = iter.next().unwrap();
        let v1: usize = iter.next().unwrap().parse().unwrap();

        match v0 {
            "SET" => {
                let v2: isize = iter.next().unwrap().parse().unwrap();
                people[v1 - 1].0 = v2;
                people[v1 - 1].1 = last_restart;
            },
            "PRINT" => {
                if people[v1 - 1].1 < last_restart {
                    people[v1 - 1].0 = last_restart_v;
                    people[v1 - 1].1 = last_restart;
                }
                writeln!(&mut w, "{}", people[v1 - 1].0).unwrap();
            },
            "RESTART" => {
                last_restart = i as isize;
                last_restart_v = v1 as isize;
            },
            _ => panic!()
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bokforing_sampleinputs() {
        for mut file in std::fs::read_dir("input/bokforing")
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
