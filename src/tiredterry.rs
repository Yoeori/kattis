use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines =  input.lines();
    let first_line = lines.next().unwrap().unwrap().split(' ').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let (n, p, d) = (first_line[0], first_line[1], first_line[2]);

    let time_period: Vec<usize> = lines.next().unwrap().unwrap().chars().map(|c| if c == 'Z' { 1 } else { 0 }).cycle().take(n*2).collect();
    let mut slept: usize = time_period[(n-p)..n].iter().sum();
    let mut count = 0;
    for i in 0..n {
        slept -= time_period[(i+n)-p];
        slept += time_period[i+n];
        if slept < d {
            count += 1;
        }
    }

    writeln!(&mut w, "{}", count).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiredterry_sampleinputs() {
        for mut file in std::fs::read_dir("input/tiredterry")
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
